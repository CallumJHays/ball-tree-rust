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
        if self != Nil {
            let (center, rad) = self._get_center_and_radius();
            let dist = distance(&center, &features);
            if dist > rad {
                return self._bounding_ball(Point(features.clone()))
            }
        }
        self._push_node(&Point(features.clone()))
    }

    pub fn nn_search(&self, features: &Vec<f32>, max_entries: &usize) -> Vec<Vec<f32>> {
        let mut list = self._nn_search_node(&features, &max_entries);
        list.sort_by(|a, b| {
            distance(&a, &features)
            .partial_cmp(
                &distance(&b, &features)
            )
            .unwrap_or(Ordering::Equal)
        });
        list
    }
    
    fn _nn_search_node(&self, features: &Vec<f32>, max_entries: &usize) -> Vec<Vec<f32>> {
        match *self {
            Point(ref center) => vec![center.clone()],
            Ball(_, _, ref left, ref right) => {
                let (left_center, left_rad) = left._get_center_and_radius();
                let (right_center, right_rad) = right._get_center_and_radius();
                let left_dist = distance(&left_center, &features);
                let right_dist = distance(&right_center, &features);

                let mut candidates: Vec<Vec<f32>> =
                    // if inside a ball
                    if left_dist <= left_rad || right_dist <= right_rad {
                        if left_dist >= left_rad {
                            right._nn_search_node(&features, &max_entries)
                        } else {
                            left._nn_search_node(&features, &max_entries)
                        }
                    } else { // choose the closest one
                        if left_dist < right_dist {
                            left._nn_search_node(&features, &max_entries)
                        } else {
                            right._nn_search_node(&features, &max_entries)
                        }
                    };

                if candidates.len() < *max_entries {
                    if left_dist < right_dist {
                        candidates.append(&mut right._nn_search_node(&features, &max_entries));
                    } else {
                        candidates.append(&mut left._nn_search_node(&features, &max_entries));
                    }
                }

                if candidates.len() > *max_entries { candidates[0..*max_entries].to_vec() } else { candidates }
            },
            Nil => Vec::new()
        }
    }

    fn _push_node(self, node: &BallTree) -> BallTree {
        match self {
            Nil => node.clone(),
            Ball(self_center, self_rad, left, right) => match *node {
                Nil => Nil,
                Point(ref node_center) => {
                    let (left_center, left_rad) = left._get_center_and_radius();
                    let (right_center, right_rad) = right._get_center_and_radius();
                    let left_dist = distance(&left_center, &node_center);
                    let right_dist = distance(&right_center, &node_center);

                    // if inside either ball, choose which ball to insert the node into
                    let (new_left, new_right) = if left_dist <= left_rad || right_dist <= right_rad {
                        if left_dist <= left_rad {
                            (Box::new(left._push_node(node)), right)
                        } else {
                            (left, Box::new(right._push_node(node)))
                        }
                    } else {
                        // node is in neither left nor right, wrap in new ball with the closest child
                        if left_dist < right_dist {
                            (Box::new(left._bounding_ball(node.clone())), right)
                        } else {
                            (left, Box::new(right._bounding_ball(node.clone())))
                        }
                    };
                    Ball(self_center, self_rad, new_left, new_right)
                },
                Ball(_, _, _, _) => panic!("Adding entire balls to ball tree is not supported!")
            },
            Point(self_center) => Point(self_center)._bounding_ball(node.clone())
        }
    }

    pub fn _bounding_ball(self, other: BallTree) -> BallTree {
        let (self_center, self_rad) = self._get_center_and_radius();
        let (other_center, other_rad) = other._get_center_and_radius();

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
