pub mod vector_math;

#[cfg(test)]
mod tests;

extern crate rayon;

use std::marker::PhantomData;
use std::mem;
use std::cmp::Ordering;

pub trait HasMeasurableDiff {
    fn difference(&self, other: &Self) -> f32;
    fn midpoint(&self, other: &Self, self_rad: f32, other_rad: f32) -> Self;
}

// TODO
// pub trait IsKdConstructable<V: Sync> where Self: Sync + Sized + HasMeasurableDiff {
//     fn construct(collection: Vec<(Self, V)>) -> BallTree<Self, V>;
// }

#[derive(Debug, PartialEq)]
pub enum Ball<K, V> where K: HasMeasurableDiff + Sync, V: Sync  {
    Stub,
    Leaf { key: K, val: V},
    Branch {
        key: K, radius: f32, size: u32,
        left: Box<Ball<K, V>>,
        right: Box<Ball<K, V>>,
        _val_type: PhantomData<V>,
    }
}

pub struct BallTree<K, V> where K: HasMeasurableDiff + Sync, V: Sync {
    root: Option<Box<Ball<K, V>>>,
    size: u32,
    _key_type: PhantomData<K>,
    _val_type: PhantomData<V>,
}

impl<K, V> Ball<K, V> where K: HasMeasurableDiff + Sync, V: Sync {
    pub fn new(key: K, val: V) -> Self {
        Ball::Leaf { key: key, val: val }
    }

    pub fn key(&self) -> &K {
        match *self {
            Ball::Leaf { ref key, .. } => key,
            Ball::Branch { ref key, .. } => key,
            Ball::Stub => panic!()
        }
    }

    fn get_key_and_radius(&self) -> (&K, f32) {
        match *self {
            Ball::Leaf { ref key, .. } => (key, 0.),
            Ball::Branch { ref key, ref radius, .. } => (key, radius.clone()),
            Ball::Stub => panic!()
        }
    }

    fn size(&self) -> u32 {
        match *self {
            Ball::Leaf { .. } => 1,
            Ball::Branch { ref size, .. } => size.clone(),
            Ball::Stub => panic!()
        }
    }

    fn nn_search_node(&self, search_key: &K, limit: u32) -> Vec<&Self> {
        // traverse the tree iteratively until one child is not large enough
        // to satisfy limit, or until limit is fulfilled
        let mut cur_node: *const Ball<K, V> = self as *const Ball<K, V>;
        let mut go_left;
        loop {
            match unsafe { &*cur_node } {
                ref leaf @ &Ball::Leaf { .. } => return vec![leaf],
                &Ball::Branch { ref left, ref right, .. } => {
                    // choose the best child to search
                    go_left = left.key().difference(&search_key) <= right.key().difference(&search_key);

                    let (closest, furthest) = if go_left { (left, right) } else { (right, left) };
                    
                    if closest.size() < limit {
                        // parrallellize the remaining search among both children
                        let (mut r1, mut r2) = rayon::join(
                            || closest.nn_search_node(&search_key, closest.size()),
                            || furthest.nn_search_node(&search_key, (limit - closest.size()))
                        );
                        r1.append(&mut r2);
                        return r1;
                    } else {
                        cur_node = &**closest as *const Ball<K, V>;
                    }
                },
                &Ball::Stub => panic!()
            }
        }
    }
}

impl<K, V> BallTree<K, V> where K: HasMeasurableDiff + Sync, V: Sync {
    pub fn new() -> Self {
        BallTree {
            root: None,
            size: 0,
            _key_type: PhantomData,
            _val_type: PhantomData
        }
    }

    pub fn size(&self) -> u32 { self.size }

    pub fn nn_search(&self, search_key: &K, limit: u32) -> Vec<&Ball<K, V>> {
        let root_node = match self.root {
            None => return vec![],
            Some(ref root_node) => &**root_node
        };

        // return search results ordered by difference
        let mut results = root_node.nn_search_node(&search_key, limit);
        results.sort_by(|n1, n2| {
            let n1_diff = n1.key().difference(&search_key);
            let n2_diff = n2.key().difference(&search_key);
            n1_diff.partial_cmp(&n2_diff).unwrap_or(Ordering::Equal)
        });
        results
    }

    pub fn push(&mut self, node: Ball<K, V>) {
        self.size += 1;

        // if root is none, make new node the new root
        let mut root_node = match self.root.take() {
            None => return self.root = Some(Box::new(node)),
            Some(root_node) => root_node,
        };

        // handle if outside root ball initially
        {
            let outside_root: bool = {
                let (root_key, root_rad) = root_node.get_key_and_radius();
                root_key.difference(node.key()) > root_rad
            };

            if outside_root {
                return self.root = Some(Box::new(bounding_ball(node, *root_node)))
            }
        }
        
        // search iteratively until bounded with the closest ball
        let mut cur_child: *mut Box<Ball<K, V>> = &mut root_node;
        loop {
            if let &mut Ball::Branch { ref mut size, ref mut left, ref mut right, .. } = unsafe { &mut **cur_child } {
                *size += 1;
                let (go_left, outside_both) = {
                    let (left_key, left_rad) = left.get_key_and_radius();
                    let (right_key, right_rad) = right.get_key_and_radius();
                    let node_key = node.key();
                    let left_diff = left_key.difference(&node_key);
                    let right_diff = right_key.difference(&node_key);

                    // outside_both if not in either ball
                    let outside_both = left_diff > left_rad && right_diff > right_rad;
                    let go_left = left_diff <= if outside_both { right_diff } else { left_rad };
                    (go_left, outside_both)
                };

                if outside_both {
                    let closest_child = if go_left { left } else { right };
                    let old_child = mem::replace(closest_child, Box::new(Ball::Stub));
                    mem::replace(closest_child, Box::new(bounding_ball(*old_child, node)));
                    return self.root = Some(root_node);
                } else {
                    cur_child = if go_left { left } else { right };
                }
            } else { panic!() }
        }
    }
}

fn bounding_ball<K, V>(b1: Ball<K, V>, b2: Ball<K, V>) -> Ball<K, V>
        where K: HasMeasurableDiff + Sync, V: Sync {

    let (midpoint, radius) = {
        let (b1_key, b1_rad) = b1.get_key_and_radius();
        let (b2_key, b2_rad) = b2.get_key_and_radius();

        let midpoint = b1_key.midpoint(&b2_key, b1_rad, b2_rad);
        let radius = b1_key.difference(&midpoint) + b1_rad;

        if radius.is_nan() { panic!("radius was NaN! Please thoroughly test your midpoint function.") }

        (midpoint, radius)
    };

    Ball::Branch {
        key: midpoint,
        radius: radius,
        size: b1.size() + b2.size(),
        left: Box::new(b1), right: Box::new(b2),
        _val_type: PhantomData
    }
}
