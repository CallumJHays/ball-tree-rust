use std::cmp::Ordering;
use vector_math::*;
use self::BallTree::*; // shorthand enum

pub trait Baller {
    fn metric(&self, &Self) -> f32;
    fn midpoint(&self, &f32, &Self, &f32) -> Self;
}

impl Baller for Vec<f32> {
    fn metric(&self, other: &Vec<f32>) -> f32 {
        distance(&self, &other)
    }

    fn midpoint(&self, self_rad: &f32, other: &Vec<f32>, other_rad: &f32) -> Vec<f32> {
        let span = subtract_vec(&self, &other);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self, &multiply_scal(&unit_vec, &self_rad));
        let p2 = subtract_vec(&other, &multiply_scal(&unit_vec, &other_rad));
        midpoint(&p2, &p1)
    }
}

// An immutable hyperdimensional ball tree
#[derive(Clone, Debug, PartialEq)]
pub enum BallTree<T: Baller + Clone> {
    Point(T),
    Ball(T, f32, Box<BallTree<T>>, Box<BallTree<T>>),
    Nil,
}

impl<T: Baller + Clone> BallTree<T> {
    pub fn new() -> Self { Nil }

    pub fn load(collection: Vec<T>) -> Self {
       BallTree::_load_push(collection)
    }

    pub fn flatten(&self) -> Vec<T> {
        self._flatten_node()
    }

    pub fn push(&mut self, item: &T) {
        let new_node = Point(item.clone());

        match *self {
            Nil => {},
            _ => {
                let (key, rad) = self._get_key_and_radius();
                let dist = key.metric(&item);
                if dist > rad {
                    return self._bounding_ball(new_node);
                }
            }
        }
        
        let mut current_self = self;
        loop {
            match *current_self {
                Nil => {
                    *current_self = new_node;
                    break;
                },
                Ball(ref self_key, ref self_rad, ref left, ref right) => match new_node {
                    Nil => {},
                    Point(_) => {
                        let (left_key, left_rad) = left._get_key_and_radius();
                        let (right_key, right_rad) = right._get_key_and_radius();
                        let left_dist = item.metric(&left_key);
                        let right_dist = item.metric(&right_key);

                        // if inside either ball, choose which ball to insert the item into
                        if left_dist <= left_rad || right_dist <= right_rad {
                            current_self = if left_dist < right_dist { &mut left } else { &mut right };
                        } else {
                            // item is in neither left nor right, wrap in new ball with the closest child
                            if left_dist <= left_rad {
                                left._bounding_ball(new_node);
                            } else {
                                right._bounding_ball(new_node);
                            }
                            // we have the answer. we can return now.
                            break;
                        }
                    },
                    Ball(_, _, _, _) => panic!("Adding entire balls to ball tree is not supported!")
                },
                Point(self_key) => {
                    current_self._bounding_ball(new_node);
                    break;
                }
            }
        }
    }

    pub fn nn_search(&self, item: &T, max_entries: &usize) -> Vec<T> {
        let mut list = self._nn_search_node(&item, &max_entries);
        list.sort_by(|a, b| {
            a.metric(&item)
            .partial_cmp(&b.metric(&item))
            .unwrap_or(Ordering::Equal)
        });
        list
    }

    fn _load_push(collection: Vec<T>) -> Self {
        let mut bt = BallTree::new();
        for item in collection.into_iter() {
            bt.push(&item);
        }
        bt
    }

    fn _flatten_node(&self) -> Vec<T> {
        match *self {
            Nil => Vec::new(),
            Point(ref x) => vec![x.clone()],
            Ball(_, _, ref left, ref right) => {
                let mut points = left._flatten_node();
                points.append(&mut right._flatten_node());
                points
            }
        }
    }
    
    fn _nn_search_node(&self, item: &T, max_entries: &usize) -> Vec<T> {
        match *self {
            Point(ref center) => vec![center.clone()],
            Ball(_, _, ref left, ref right) => {
                let (left_key, left_rad) = left._get_key_and_radius();
                let (right_key, right_rad) = right._get_key_and_radius();
                let left_dist = item.metric(&left_key);
                let right_dist = item.metric(&right_key);

                let mut candidates: Vec<T> =
                    if left_dist <= left_rad || right_dist <= right_rad { // if inside either ball
                        if left_dist <= left_rad {
                            left._nn_search_node(&item, &max_entries)
                        } else {
                            right._nn_search_node(&item, &max_entries)
                        }
                    } else { // choose the closest one
                        if left_dist < right_dist {
                            left._nn_search_node(&item, &max_entries)
                        } else {
                            right._nn_search_node(&item, &max_entries)
                        }
                    };

                if candidates.len() < *max_entries {
                    if left_dist < right_dist {
                        candidates.append(&mut right._nn_search_node(&item, &max_entries));
                    } else {
                        candidates.append(&mut left._nn_search_node(&item, &max_entries));
                    }
                }

                if candidates.len() > *max_entries { candidates[0..*max_entries].to_vec() } else { candidates }
            },
            Nil => Vec::new()
        }
    }

    pub fn _bounding_ball(&mut self, other: Self) {
        let (self_key, self_rad) = self._get_key_and_radius();
        let (other_key, other_rad) = other._get_key_and_radius();
        let midpoint = self_key.midpoint(&self_rad, &other_key, &other_rad);
        *self = Ball(
            midpoint.clone(),
            self_key.metric(&midpoint) + self_rad,
            Box::new(*self),
            Box::new(other))
    }

    fn _get_key_and_radius(&self) -> (T, f32) {
        match *self {
            Point(ref key) => (key.clone(), 0.),
            Ball(ref key, ref rad, _, _) => (key.clone(), *rad),
            Nil => panic!("The supplied tree is Nil!")
        }
    }
}
