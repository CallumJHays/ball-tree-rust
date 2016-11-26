use std::cell::RefCell;
use std::rc::Rc;

#[cfg(tests)]
mod tests;

// An immutable hyperdimensional ball tree
#[derive(Clone)]
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

    pub fn push(self, features: Vec<f32>) -> BallTree {
        self.push_node(Point(features))
    }

    fn push_node(self, node: BallTree) -> BallTree {
        match self {
            Nil => node.clone(),
            Ball(self_center, self_rad, left, right) => match node {
                Nil => Nil,
                Point(ref node_center) => {
                    let get_dist_rad = |tree| match tree {
                        &Point(ref center) => (distance(&node_center, center), 0.),
                        &Ball(ref center, ref rad, _, _) => (distance(&node_center, center), *rad),
                        &Nil => panic!("This ball has a Nil left or right child!")
                    };

                    let (left_dist, left_rad) = get_dist_rad(&left);
                    let (right_dist, right_rad) = get_dist_rad(&right);
                    
                    // if inside both balls, choose ball to push to based on distance
                    if left_dist < left_rad && right_dist < right_rad {
                        if left_dist < right_dist {
                            Ball(self_center, self_rad, Box::new(left.push_node(node)), right)
                        } else {
                            Ball(self_center, self_rad, left, Box::new(right.push_node(node)))
                        }
                    } else if left_dist < left_rad {
                            Ball(self_center, self_rad, Box::new(left.push_node(node)), right)
                    } else if right_dist < right_rad {
                            Ball(self_center, self_rad, left, Box::new(right.push_node(node)))
                    } else {
                        // node is in neither left nor right, wrap closest child in a new ball
                        if left_dist < right_dist {
                            let ball_wrapper = (*left).bounding_ball(node);
                            Ball(self_center, self_rad, Box::new(ball_wrapper), right)
                        } else {
                            let ball_wrapper = (*right).bounding_ball(node);
                            Ball(self_center, self_rad, left, Box::new(ball_wrapper))
                        }
                    }
                },
                Ball(ref node_center, ref node_rad, _, _) => {
                    Nil
                }
            },
            Point(self_center) => {
                Nil
            }
        }
    }

    fn bounding_ball(self, other: BallTree) -> BallTree {
        let get_center_rad = |tree| match tree {
            &Point(ref center) => (center, 0.),
            &Ball(ref center, ref rad, _, _) => (center, *rad),
            &Nil => panic!("Bounding ball called on a Nil balltree")
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
    v1.into_iter().map(|x| x.powi(2)).fold(0., |sum, x| sum + x).sqrt()
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
