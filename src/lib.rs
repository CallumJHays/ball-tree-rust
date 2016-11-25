use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

#[cfg(tests)]
mod tests;

// A hyperdimensional ball that may have other balls as children.
// should be created with the Ball::new()
pub struct Ball {
    pub center: Vec<f32>,
    pub radius: f32,
    pub parent: Option<Rc<RefCell<Ball>>>,
    pub left_child: Option<Rc<RefCell<Ball>>>,
    pub right_child: Option<Rc<RefCell<Ball>>>
}

// useful vector functions
// assume that vector to vector operations are performed with vectors of the same size
fn add_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + v2[i]).collect()
}
fn add_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + scalar).collect()
}

fn subtract_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - v2[i]).collect()
}
fn subtract_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - scalar).collect()
}

fn multiply_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x * scalar).collect()
}

fn divide_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x / scalar).collect()
}

fn magnitude(v1: &Vec<f32>) -> f32 {
    v1.into_iter().map(|x| x.powi(2)).fold(0., |sum, x| sum + x)
}

fn distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    (0..v1.len())
    .map(|i| (v1[i] - v2[i]).powi(2))
    .fold(0., |sum, x| sum + x)
    .sqrt()
}

fn midpoint(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len())
    .map(|i| (v1[i] + v2[i]) / 2.)
    .collect()
}

impl Ball {
    fn new(features: &Vec<f32>) -> Ball {
        Ball {
            center: features.clone(), radius: 0.,
            parent: None, left_child: None, right_child: None
        }
    }

    fn bounding_ball(&self, other: &Ball) -> Rc<RefCell<Ball>> {
        let span = subtract_vec(&self.center, &other.center);
        let magnitude = magnitude(&span);
        let unit_vec = divide_scal(&span, &magnitude);
        let p1 = add_vec(&self.center, &multiply_scal(&unit_vec, &self.radius));
        let p2 = subtract_vec(&other.center, &multiply_scal(&unit_vec, &other.radius));
        let new_ball = Ball {
            center: midpoint(&p1, &p2),
            radius: distance(&p1, &p2) / 2.,
            parent: None, left_child: None, right_child: None
        };
        Rc::new(RefCell::new(new_ball))
    }

    fn insert(&self, new_ball: Rc<RefCell<Ball>>) {
        let dist = distance(&self.center, &new_ball.center);
        // if outside of the ball, make a new parent ball and put both inside
        if dist > self.radius {
            let new_parent = self.bounding_ball(&new_ball);
            new_parent.parent = self.parent;
            new_parent.left_child = Some(&mut Box::new(*self));
            new_parent.right_child = Some(new_ball);
            self.parent = Some(&mut new_parent);
        }

        // if the ball contains other balls
        // if self.radius > 0. {

        //     let left_child = self.left_child.unwrap();
        //     let right_child = self.right_child.unwrap();

        //     // calculate the closest ball to insert the new ball into
        //     let left_dist = new_ball.distance(&left_child);
        //     let right_dist = new_ball.distance(&right_child);
        //     if left_dist < right_dist {
        //         left_child.insert(&new_ball);
        //     } else {
        //         right_child.insert(&new_ball);
        //     }
        // } else {
            
        // }
    }
}
