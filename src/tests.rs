extern crate rand;

use super::*;
use self::rand::Rng;

type Feature = Vec<f32>;

fn simple_sample_features() -> [Feature; 7] {
    [
        vec![15., -7.],
        vec![-20., -20.],
        vec![9., 10.],
        vec![6., 6.],
        vec![-2., 10.],
        vec![18., 5.],
        vec![1., 1.],
    ]
}

fn simple_sample_tree() -> BallTree<Feature, i32> {
    let mut bt = BallTree::new();
    let mut i = 1;
    for feature in simple_sample_features().into_iter() {
        bt.push(Ball::new(feature.clone(), i));
        i += 1;
    }

    bt
}

impl HasMeasurableDiff for Feature {
    fn difference(&self, other: &Self) -> f32 {
        (0..self.len())
        .map(|i| (self[i] - other[i]).powi(2))
        .fold(0., |sum, x| sum + x)
        .sqrt()
    }

    fn midpoint(&self, other: &Self, self_rad: f32, other_rad: f32) -> Self {
        // span = self - other
        let span: Feature = (0..self.len()).map(|i| self[i] - other[i]).collect();
        // mag = sqrt(sum(x^2))
        let mag = span.iter().fold(0., |sum, x| sum + x.powi(2)).sqrt();
        // unit_vec = 1 unit from other to self
        let unit_vec: Feature = span.into_iter().map(|x| x / mag).collect();
        // self_off = unit_vec * self_rad
        let self_off: Feature = unit_vec.iter().map(|x| x * self_rad).collect();
        // other_off = unit_vec * other_rad
        let other_off: Feature = unit_vec.into_iter().map(|x| -x * other_rad).collect();
        // self_p = self + self_off
        let self_p: Feature = (0..self.len()).map(|i| self[i] + self_off[i]).collect();
        // other_p = other + other_off
        let other_p: Feature = (0..self.len()).map(|i| other[i] + other_off[i]).collect();
        // midpoint = (self_p + other_p) / 2
        (0..self.len()).map(|i| (self_p[i] + other_p[i]) / 2.).collect()
    }
}

fn rand_feature(dimensions: u32) -> Feature {
    let mut rng = rand::thread_rng();
    (0..(2 as u32).pow(dimensions))
    .map(|_| rng.gen::<f32>())
    .collect()
}

#[test]
fn midpoint_impl() {
    let test = vec![50., 10., 10.].midpoint(&vec![-50., 10., 10.], 25., 25.);
    assert_eq!(test, vec![0., 10., 10.]);
}

#[test]
fn nn_search() {
    // searching an empty tree gets an empty vec
    let bt: BallTree<Feature, i32> = BallTree::new();
    let search_feature = vec![10., 10.];
    assert_eq!(bt.nn_search(&search_feature, 1), vec![] as Vec<&Ball<Feature, i32>>);

    // searching a tree gets the desired results
    let expected_res = vec![
        Ball::new(vec![9., 10.], 3),
        Ball::new(vec![6., 6.], 4),
        Ball::new(vec![18., 5.], 6),
        Ball::new(vec![-2., 10.], 5),
        Ball::new(vec![1., 1.], 7),
    ];
    let expected_res_ref: Vec<&Ball<Feature, i32>> = expected_res.iter().collect();
    let bt = simple_sample_tree();
    assert_eq!(bt.nn_search(&search_feature, 5), expected_res_ref);
}

#[test]
fn push() {
    let mut bt = BallTree::new();
    assert_eq!(bt.size(), 0);

    for i in 0..20 {
        bt.push(Ball::new(rand_feature(9), i));
        assert_eq!(bt.size(), i + 1);
    }
}
