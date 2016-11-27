use std::cmp::Ordering;
use vector_math::*;
use self::BallTree::*; // shorthand enum

// An immutable hyperdimensional ball tree
#[derive(Clone, Debug, PartialEq)]
pub enum BallTree {
    Point(Vec<f32>),
    //     center,   rad, left,      right
    Ball(Vec<f32>, f32, Box<BallTree>, Box<BallTree>),
    Nil,
}

impl BallTree {
    pub fn new() -> BallTree { Nil }

    pub fn push(self, features: &Vec<f32>) -> BallTree {
        self.push_node(&Point(features.clone()))
    }

    pub fn nn_search(&self, features: &Vec<f32>, max_entries: &usize) -> Vec<Vec<f32>> {
        let mut list = self.nn_search_node(&features, &max_entries);
        list.sort_by(|a, b| {
            distance(&a, &features)
            .partial_cmp(
                &distance(&b, &features)
            )
            .unwrap_or(Ordering::Equal)
        });
        list
    }
    
    fn nn_search_node(&self, features: &Vec<f32>, max_entries: &usize) -> Vec<Vec<f32>> {
        match *self {
            Point(ref center) => vec![center.clone()],
            Ball(_, _, ref left, ref right) => {
                let get_dist = |tree: &BallTree| match *tree {
                    Point(ref center) | Ball(ref center, _, _, _) => distance(features, &center),
                    Nil => panic!("The supplied tree us Nil!")
                };
                let left_dist = get_dist(&left);
                let right_dist = get_dist(&right);

                let mut candidates: Vec<Vec<f32>> = if left_dist < right_dist {
                    left.nn_search(&features, &max_entries)
                } else {
                    right.nn_search(&features, &max_entries)
                };

                if candidates.len() < *max_entries {
                    if left_dist < right_dist {
                        candidates.append(&mut right.nn_search(&features, &max_entries));
                    } else {
                        candidates.append(&mut left.nn_search(&features, &max_entries));
                    }
                }

                if candidates.len() > *max_entries { candidates[0..*max_entries].to_vec() } else { candidates }
            },
            Nil => Vec::new()
        }
    }

    fn push_node(self, node: &BallTree) -> BallTree {
        match self {
            Nil => node.clone(),
            Ball(self_center, self_rad, left, right) => match *node {
                Nil => Nil,
                Point(ref node_center) => {
                    let (left_center, left_rad) = left._get_center_and_radius();
                    let (right_center, right_rad) = right._get_center_and_radius();
                    let left_dist = distance(&left_center, &node_center);
                    let right_dist = distance(&right_center, &node_center);

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
                Ball(_, _, _, _) => panic!("Adding entire balls to ball tree is not supported!")
            },
            Point(self_center) => Point(self_center).bounding_ball(node.clone())
        }
    }

    fn bounding_ball(self, other: BallTree) -> BallTree {
        let (self_center, self_rad) = self._get_center_and_radius();
        let (other_center, other_rad) = self._get_center_and_radius();

        let span = subtract_vec(&self_center, &other_center);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self_center, &multiply_scal(&unit_vec, &self_rad));
        let p2 = subtract_vec(&other_center, &multiply_scal(&unit_vec, &other_rad));
        Ball(midpoint(&p1, &p2), distance(&p1, &p2) / 2., Box::new(self), Box::new(other))
    }

    fn _get_center_and_radius(&self) -> (Vec<f32>, f32) {
        match *self {
            Point(ref center) => (center.clone(), 0.),
            Ball(ref center, ref rad, _, _) => (center.clone(), *rad),
            Nil => panic!("The supplied tree is Nil!")
        }
    }
}
