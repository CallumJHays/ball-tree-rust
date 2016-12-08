extern crate ball_tree;
extern crate rand;
extern crate simd;
extern crate byteorder;

use self::rand::Rng;
use ball_tree::*;
use std::time::Instant;
use std::fs::File;
use std::io::Write;
use self::byteorder::{BigEndian, WriteBytesExt};
use self::simd::f32x4;

pub struct Feature {
    key: [f32; 8]
}

impl Feature {
    pub fn new(key: [f32; 8]) -> Self {
        Feature { key: key }
    }
}

trait Convertible {
    fn as_array(&self) -> [f32; 8];
}

impl Convertible for Vec<f32> {
    fn as_array(&self) -> [f32; 8] {
        assert_eq!(self.len(), 8);
        let mut arr = [0.; 8];
        let mut i = 0;
        for x in self.iter() {
            arr[i] = x.clone();
            i += 1;
        }
        arr
    }
}

impl HasMeasurableDiff for Feature {
    
    fn difference(&self, other: &Self) -> f32 {
        // convert self and other to array counterparts
        let x = self.key;
        let y = other.key;

        assert_eq!(x.len(), y.len());

        let len = x.len();
        let mut sum = f32x4::splat(0.0);
        let mut i = 0;
        while i < len & !3 {
            let x = f32x4::load(&x, i);
            let y = f32x4::load(&y, i);
            sum = sum + (x - y) * (x - y);
            i += 4
        }
        sum.extract(0) + sum.extract(1) + sum.extract(2) + sum.extract(3)
    }

    fn midpoint(&self, other: &Self, self_rad: f32, other_rad: f32) -> Self {
        // span = self - other
        let span: Vec<f32> = (0..self.key.len()).map(|i| self.key[i] - other.key[i]).collect();
        // mag2 = sum(x^2)
        let mag2 = span.iter().fold(0., |sum, x| sum + x.powi(2));
        // unit_vec = 1 unit from other to self
        let unit_vec: Vec<f32> = span.into_iter().map(|x| x / mag2).collect();
        // self_off = unit_vec * self_rad
        let self_off: Vec<f32> = unit_vec.iter().map(|x| x * self_rad).collect();
        // other_off = unit_vec * other_rad
        let other_off: Vec<f32> = unit_vec.into_iter().map(|x| -x * other_rad).collect();
        // self_p = self + self_off
        let self_p: Vec<f32> = (0..self.key.len()).map(|i| self.key[i] + self_off[i]).collect();
        // other_p = other + other_off
        let other_p: Vec<f32> = (0..self.key.len()).map(|i| other.key[i] + other_off[i]).collect();
        // midpoint = (self_p + other_p) / 2
        Feature {
            key: (0..self.key.len()).map(|i| (self_p[i] + other_p[i]) / 2.).collect::<Vec<f32>>().as_array()
        }
    }
}

pub fn rand_feature(dimensions: u32) -> Feature {
    let mut rng = rand::thread_rng();
    let key = (0..(2 as u32).pow(dimensions))
    .map(|_| rng.next_f32())
    .collect::<Vec<f32>>().as_array();

    Feature { key: key }
}

pub fn rand_balltree(size: u32, dimensions: u32) -> BallTree<Feature, u32> {
    println!("rand_balltree called");

    let mut bt: BallTree<Feature, u32> = BallTree::new();
    let num = (2 as u32).pow(size);
    let mut save_file = File::create(format!("benchmarks/{}.blob", dimensions)).unwrap();
    let mut now = Instant::now();
    for i in 0..num {
        bt.push(Ball::new(rand_feature(dimensions), i + 1));
        if i % 100 == 0 {
            let push_time = now.elapsed();
            let search_key = rand_feature(dimensions);
            now = Instant::now();
            bt.nn_search(&search_key, 100);
            let search_time = now.elapsed();
            println!(
                "{}/{} loaded -- push time: {}s, {}ns -- search time: {}s, {}ns",
                i, num, push_time.as_secs(), push_time.subsec_nanos(), search_time.as_secs(), search_time.subsec_nanos());
            let mut buffer = vec![];
            buffer.write_u32::<BigEndian>(i);
            buffer.write_u64::<BigEndian>(push_time.as_secs());
            buffer.write_u32::<BigEndian>(push_time.subsec_nanos());
            buffer.write_u64::<BigEndian>(search_time.as_secs());
            buffer.write_u32::<BigEndian>(search_time.subsec_nanos());
            save_file.write(&buffer);
            now = Instant::now();
        }
    }
    bt
}

fn zeros(dims: u32) -> Vec<f32> {
    (0..(2 as u32).pow(dims)).map(|_| 0.).collect()
}
