use std::marker::PhantomData;
use std::fmt::Debug;
use std::mem;
use std::thread;

pub trait HasMeasurableDiff {
    fn difference(&self, other: &Self) -> f32;

    fn midpoint(&self, other: &Self, self_rad: &f32, other_rad: &f32) -> Self;
}

#[derive(Debug, PartialEq)]
pub enum Ball<K, V> where K: HasMeasurableDiff  {
    Stub,
    Leaf { key: K, val: V},
    Branch {
        key: K, radius: f32, size: u32,
        left: Box<Ball<K, V>>,
        right: Box<Ball<K, V>>,
        _val_type: PhantomData<V>,
    }
}

pub struct BallTree<K, V> where K: HasMeasurableDiff  {
    root: Option<Box<Ball<K, V>>>,
    size: u32,
    _key_type: PhantomData<K>,
    _val_type: PhantomData<V>,
}

impl<K, V> Ball<K, V> where K: HasMeasurableDiff {
    pub fn new(key: K, val: V) -> Self {
        Ball::Leaf { key: key, val: val }
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
}

impl<K, V> BallTree<K, V> where K: HasMeasurableDiff + Debug, V: Debug {
    pub fn new() -> Self {
        BallTree {
            root: None,
            size: 0,
            _key_type: PhantomData,
            _val_type: PhantomData
        }
    }

    pub fn nn_search(&self, search_key: &K, limit: &u32) -> Vec<&Ball<K, V>> {
        let root_node = match self.root {
            None => return vec![],
            Some(ref root_node) => root_node
        };

        // return search results ordered by difference
        root_node._nn_search_node(&search_key, &limit)
        .sort_by(|node, _| {
            let (node_key, _) = node.get_key_and_radius();
            node_key.difference(&search_key)
        })
    }

    fn _nn_search_node(&self, search_key: &K, limit: &u32) -> Vec<&Ball<K, V>> {
        // traverse the tree iteratively until one child is not large enough
        // to satisfy limit, or until limit is fulfilled
        let mut cur_node: *const Box<Ball<K, V>> = &self;
        let mut go_left;
        loop {
            match unsafe { &**cur_node } {
                leaf @ Ball::Leaf { .. } => return vec![leaf],
                Ball::Branch { ref left, ref right, .. } => {
                    // choose the best child to search
                    go_left = {
                        let (left_key, left_rad) = left.get_key_and_radius();
                        let (right_key, right_rad) = right.get_key_and_radius();
                        let left_diff = left_key.difference(&search_key);
                        let right_diff = right_key.difference(&search_key);

                        if left_diff <= left_rad || right_diff <= right_rad {
                            left_diff <= right_diff
                        } else {
                            left_diff <= left_rad
                        }
                    };

                    let closest_child = if go_left { left } else { right };

                    if closest.size() < limit {
                        break; // break out of the loop and start parrallellizing the search
                    } else {
                        cur_node = &closest;
                    }
                },
                Ball::Stub => panic!()
            }
        }

        let (closest, furthest) = if go_left { (left, right) } else { (right, left) };

        // spawn a thread to search the closest
        let closestResultsThread = thread::spawn(move || {
            closest._nn_search_node(&search_key, &closest.size())
        });
        // search the furthest ourself
        let furthestResults = furthest._nn_search_node(&search_key, &(limit - closest.size()));
        // join and return the results
        closestResultsThread.join() ++ furthestResults
    }

    pub fn push(&mut self, mut node: Ball<K, V>) {
        let node_ptr: *mut Ball<K, V> = &mut node;
        self.size += 1;

        // if root is none, make new node the new root
        let mut root_node = match self.root.take() {
            None => return self.root = Some(Box::new(node)),
            Some(root_node) => root_node,
        };

        // handle if outside root ball initially
        {
            let outside_root: bool = {
                let (node_key, _) = node.get_key_and_radius();
                let (root_key, root_rad) = root_node.get_key_and_radius();
                root_key.difference(node_key) > root_rad
            };

            if outside_root {
                return self.root = Some(Box::new(bounding_ball(node, *root_node)))
            }
        }
        
        // search iteratively until bounded with the closest ball
        let mut cur_child: *mut Box<Ball<K, V>> = &mut root_node;
        loop {
            if let &mut Ball::Branch { ref mut left, ref mut right, .. } = unsafe { &mut **cur_child } {
                let (go_left, to_bound) = {
                    let (left_key, left_rad) = left.get_key_and_radius();
                    let (right_key, right_rad) = right.get_key_and_radius();
                    let (node_key, _) = node.get_key_and_radius();
                    let left_diff = left_key.difference(&node_key);
                    let right_diff = right_key.difference(&node_key);

                    // to_bound if not in either ball
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
            }
        }
    }
}

fn bounding_ball<K, V>(b1: Ball<K, V>, b2: Ball<K, V>) -> Ball<K, V>
        where K: HasMeasurableDiff + Debug {

    let (midpoint, radius) = {
        let (b1_key, b1_rad) = b1.get_key_and_radius();
        let (b2_key, b2_rad) = b2.get_key_and_radius();

        let midpoint = b1_key.midpoint(&b2_key, &b1_rad, &b2_rad);
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

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use self::rand::Rng;

    type Feature = Vec<f32>;

    fn rand_feature(dimensions: usize) -> Feature {
        let mut rng = rand::thread_rng();
        (0..dimensions)
        .map(|_| rng.gen::<f32>())
        .collect()
    }

    impl HasMeasurableDiff for Feature {
        fn difference(&self, other: &Self) -> f32 {
            (0..self.len())
            .map(|i| (self[i] - other[i]).powi(2))
            .fold(0., |sum, x| sum + x)
            .sqrt()
        }

        fn midpoint(&self, other: &Self, self_rad: &f32, other_rad: &f32) -> Self {
            let span: Feature = (0..self.len()).map(|i| self[i] - other[i]).collect(); // span = self - other
            let mag = span.iter().fold(0., |sum, x| sum + x.powi(2)).sqrt(); // mag = sqrt(sum(x^2))
            let unit_vec: Feature = span.into_iter().map(|x| x / mag).collect(); // unit_vec = 1 unit in dir (self - other)
            let self_off: Feature = unit_vec.iter().map(|x| x * self_rad).collect(); // self_off = unit_vec * self_rad
            let other_off: Feature = unit_vec.into_iter().map(|x| -x * other_rad).collect(); // other_off = unit_vec * other_rad
            let self_p: Feature = (0..self.len()).map(|i| self[i] + self_off[i]).collect(); // self_p = self + self_off
            let other_p: Feature = (0..self.len()).map(|i| other[i] + other_off[i]).collect(); // other_p = other + other_off
            (0..self.len()).map(|i| (self_p[i] + other_p[i]) / 2.).collect() // midpoint = (self_p + other_p) / 2
        }
    }

    #[test]
    fn midpoint() {
        let test = vec![50., 10., 10.].midpoint(&vec![-50., 10., 10.], &25., &25.);
        assert_eq!(test, vec![0., 10., 10.]);
    }

    #[test]
    fn basics() {
        let mut bt: BallTree<Feature, u32> = BallTree::new();
        assert_eq!(bt.root, None);
        assert_eq!(bt.size, 0);

        let node = Ball::new(rand_feature(512), 1);
        bt.push(node);
        assert_eq!(bt.size, 1);

        let node = Ball::new(rand_feature(512), 2);
        bt.push(node);
        assert_eq!(bt.size, 2);

        let node = Ball::new(rand_feature(513), 3);
        bt.push(node);
        assert_eq!(bt.size, 3);

        let node = Ball::new(rand_feature(512), 4);
        bt.push(node);
        assert_eq!(bt.size, 4);

        let node = Ball::new(rand_feature(512), 5);
        bt.push(node);
        assert_eq!(bt.size, 5);

        let node = Ball::new(rand_feature(512), 6);
        bt.push(node);
        assert_eq!(bt.size, 6);

        let node = Ball::new(rand_feature(512), 7);
        bt.push(node);
        assert_eq!(bt.size, 7);

        let node = Ball::new(rand_feature(512), 8);
        bt.push(node);
        assert_eq!(bt.size, 8);

        let node = Ball::new(rand_feature(512), 9);
        bt.push(node);
        assert_eq!(bt.size, 9);
    }
}
