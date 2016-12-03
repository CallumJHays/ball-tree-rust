use std::marker::PhantomData;

pub trait IsBall<K, V> {
    fn get_center_and_radius(&self) -> (&K, f32);
}

pub trait Baller {
    fn difference(&self, other: &Self) -> f32;

    fn midpoint(&self, other: &Self) -> Self;
}

pub struct BallTree<K: Baller, V, Node: IsBall<K, V>> {
    root: Option<*mut Node>,
    size: usize,
    key_type: PhantomData<K>,
    val_type: PhantomData<V>,
}

struct Ball<K: Baller, V, Node: IsBall<K, V>> {
    center: K,
    radius: f32,
    left: *mut Node,
    right: *mut Node,
    val_type: PhantomData<V>,
}

impl<K, V, Node: IsBall<K, V>> IsBall<K, V> for Ball<K, V, Node> where K: Baller {
    fn get_center_and_radius(&self) -> (&K, f32) {
        (&self.center, self.radius)
    }
}

struct Leaf<K, V> {
    center: K,
    value: V
}

impl<K, V> IsBall<K, V> for Leaf<K, V> where K: Baller {
    fn get_center_and_radius(&self) -> (&K, f32) {
        (&self.center, 0.)
    }
}

impl<K, V, Node: IsBall<K, V>> BallTree<K, V, Node> where K: Baller {
    pub fn new() -> Self {
        BallTree {
            root: None,
            size: 0,
            key_type: PhantomData,
            val_type: PhantomData
        }
    }

    pub fn push(&mut self, elem: Node) {
        unimplemented!()
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

        impl Baller for [f32; 512] {
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
