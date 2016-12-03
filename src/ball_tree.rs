use std::marker::PhantomData;

pub trait HasMeasurableDiff {
    fn difference(&self, other: &Self) -> f32;

    fn midpoint(&self, other: &Self) -> Self;
}

pub trait IsBall<K, V> where K: HasMeasurableDiff {
    fn get_key_and_radius(&self) -> (&K, f32);
}

fn bounding_ball<K, V, Node>(b1: &mut Node, b2: &mut Node) -> Ball<K, V, Node>
    where K: HasMeasurableDiff, Node: IsBall<K, V>, Ball<K, V, Node>: IsBall<K, V> {

    let (midpoint, radius) = {
        let (b1_key, b1_rad) = b1.get_key_and_radius();
        let (b2_key, b2_rad) = b2.get_key_and_radius();

        let midpoint = b1_key.midpoint(&b2_key);
        let radius = b1_key.difference(&midpoint) + b1_rad;
        (midpoint, radius)
    };

    Ball {
        key: midpoint, radius: radius,
        left: b1, right: b2,
        val_type: PhantomData
    }
}

pub struct BallTree<K: HasMeasurableDiff, V, Node: IsBall<K, V>> {
    root: Option<Box<Node>>,
    size: usize,
    key_type: PhantomData<K>,
    val_type: PhantomData<V>,
}

struct Ball<K: HasMeasurableDiff, V, Node: IsBall<K, V>> {
    key: K,
    radius: f32,
    left: *mut Node,
    right: *mut Node,
    val_type: PhantomData<V>,
}

impl<K, V, Node: IsBall<K, V>> IsBall<K, V> for Ball<K, V, Node> where K: HasMeasurableDiff {
    fn get_key_and_radius(&self) -> (&K, f32) {
        (&self.key, self.radius)
    }
}

struct Leaf<K, V> {
    key: K,
    value: V
}

impl<K, V> IsBall<K, V> for Leaf<K, V> where K: HasMeasurableDiff {
    fn get_key_and_radius(&self) -> (&K, f32) {
        (&self.key, 0.)
    }
}

impl<K, V, Node: IsBall<K, V>> BallTree<K, V, Node> where K: HasMeasurableDiff {
    pub fn new() -> Self {
        BallTree {
            root: None,
            size: 0,
            key_type: PhantomData,
            val_type: PhantomData
        }
    }

    pub fn push(&mut self, node: Leaf<K, V>) {
        let mut boxed_node = Box::new(node as Node);
        self.size += 1;

        // catch none case
        let mut root_node = match self.root {
            None => return self.root = Some(boxed_node),
            Some(ref mut root_node) => root_node,
        };

        let (node_key, _) = node.get_key_and_radius();

        // catch node outside root Ball
        // {
        //     let (root_key, root_rad) = root_node.get_key_and_radius();
        //     if root_key.difference(node_key) > root_rad {
        //         return self.root = Some(Box::new(
        //             bounding_ball(root_node, &mut node) as Node
        //         ))
        //     }
        // }
        
        // search iteratively for the right ball to bound with
        let mut new_node_raw: *mut _ = &mut *boxed_node;
        let mut cur_child_raw: *mut _ = &mut *root_node;
        
        loop {
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        struct Info {
            name: String,
            desc: String
        }

        impl HasMeasurableDiff for [f32; 512] {
            fn difference(&self, other: &Self) -> f32 {
                (0..self.len())
                .map(|i| (self[i] - other[i]).powi(2))
                .fold(0., |sum, x| sum + x)
                .sqrt()
            }

            fn midpoint(&self, other: &Self) -> Self {
                unimplemented!()
            }
        }

        // hopefully type inference will make this much less ugly
        type Key = [f32; 512];
        type Val = Info;
        type LeafNode = Leaf<Key, Val>;

        let bt: BallTree<Key, Val, Ball<Key, Val, LeafNode>> = BallTree::new();
        assert_eq!(bt.root, None);
    }
}
