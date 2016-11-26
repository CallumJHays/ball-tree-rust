use std::cell::RefCell;
use std::rc::Rc;

#[cfg(test)]
mod tests;

// An immutable hyperdimensional ball tree
#[derive(Clone, Debug, PartialEq)]
pub enum BallTree {
    Point(Vec<f32>),
    //     center,   rad, left,      right
    Ball(Vec<f32>, f32, Box<BallTree>, Box<BallTree>),
    Nil,
}

use BallTree::*;

impl BallTree {
    pub fn new() -> BallTree {
        Nil
    }

    pub fn push(self, features: &Vec<f32>) -> BallTree {
        self.push_node(&Point(features.clone()))
    }

    fn push_node(self, node: &BallTree) -> BallTree {
        match self {
            Nil => node.clone(),
            Ball(self_center, self_rad, left, right) => match node {
                &Nil => Nil,
                &Point(ref node_center) => {
                    let get_dist_rad = |tree: &BallTree| match *tree {
                        Point(ref center) => (distance(&node_center, &center), 0.),
                        Ball(ref center, rad, _, _) => (distance(&node_center, &center), rad),
                        Nil => panic!("This ball has a Nil left or right child!")
                    };

                    let (left_dist, left_rad) = get_dist_rad(&left);
                    let (right_dist, right_rad) = get_dist_rad(&right);

                    // if inside both balls, choose ball to push to based on distance
                    if left_dist <= left_rad || right_dist <= right_rad {

                        let (left_box, right_box) = if left_dist > left_rad {
                            (Box::new(left.push_node(node)), right)
                        } else if right_dist > right_rad {
                            (left, Box::new(right.push_node(node)))
                        } else {
                            if left_dist < right_dist {
                                (Box::new(left.push_node(node)), right)
                            } else {
                                (left, Box::new(right.push_node(node)))
                            }
                        };
                        Ball(self_center, self_rad, left_box, right_box)
                    } else {
                        // node is in neither left nor right, wrap in a new ball
                        let old_self = Ball(self_center, self_rad, left, right);
                        old_self.bounding_ball(node.clone())
                    }
                },
                &Ball(ref node_center, ref node_rad, _, _) => panic!("Adding entire balls to ball tree is illegal!")
            },
            Point(self_center) => Point(self_center).bounding_ball(node.clone())
        }
    }

    fn bounding_ball(self, other: BallTree) -> BallTree {
        let get_center_rad = |tree: &BallTree| match *tree {
            Point(ref center) => (center.clone(), 0.),
            Ball(ref center, rad, _, _) => (center.clone(), rad),
            Nil => panic!("Bounding ball called on a Nil balltree")
        };

        let (self_center, self_rad) = get_center_rad(&self);
        let (other_center, other_rad) = get_center_rad(&other);

        let span = subtract_vec(&self_center, &other_center);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self_center, &multiply_scal(&unit_vec, &self_rad));
        let p2 = subtract_vec(&other_center, &multiply_scal(&unit_vec, &other_rad));
        Ball(midpoint(&p1, &p2), distance(&p1, &p2) / 2., Box::new(self), Box::new(other))
    }
}

// useful vector functions
// assume that vector to vector operations are performed with vectors of the same size
pub fn add_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + v2[i]).collect()
}
pub fn add_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] + scalar).collect()
}

pub fn subtract_vec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - v2[i]).collect()
}
pub fn subtract_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    (0..v1.len()).map(|i| v1[i] - scalar).collect()
}

pub fn multiply_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x * scalar).collect()
}

pub fn divide_scal(v1: &Vec<f32>, scalar: &f32) -> Vec<f32> {
    v1.into_iter().map(|x| x / scalar).collect()
}

pub fn magnitude(v1: &Vec<f32>) -> f32 {
    v1.into_iter().map(|x| x.powi(2)).fold(0., |sum, x| sum + x).sqrt()
}

pub fn distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    (0..v1.len())
        .map(|i| (v1[i] - v2[i]).powi(2))
        .fold(0., |sum, x| sum + x)
        .sqrt()
}

pub fn midpoint(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
    (0..v1.len())
        .map(|i| (v1[i] + v2[i]) / 2.)
        .collect()
}
