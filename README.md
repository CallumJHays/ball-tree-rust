# Rust Ball-Tree implementation

Immutable Ball Tree implementation in Rust.

Ball Trees are Binary-Tree-like data structures useful for storing and searching for hyperdimensional (of more than 3 dimensions) vectors, or data that is related by a measure of difference between two points. The Ball Tree is inherently imbalanced, so lookup is likely to be **higher** than `log(n)` in the majority of cases. Ball Trees are fantastic for quickly solving nearest-neighbor and k-means clustering problems for larger datasets. They're also very applicable to many machine learning applications in production, as they allow for efficient latent feature space search, useful in recommendation systems.

## Usage
Add crate to cargo.toml
```toml
[dependencies]
ball-tree = { git = "https://github.com/callumjhays/ball-tree-rust }
```

Import crate for use:
```rust
extern crate ball_tree;

use ball_tree::ball_tree::BallTree;
```

Create a new ball tree:
```rust
let bt: BallTree<Vec<f32>> = BallTree::new()
```

Insert Items into ball tree. Note, `bt` is consumed when `push` is called on it:
```rust
let vector: Vec<f32> = vec![1., 2., 3...];
let updated_tree = bt.push(&vector);
```

Search for k nearest neighbors in a ball tree.
```rust
let search_point: Vec<f32> = vec![1., 2., 3...];
let knn: Vec<Vec<f32>> = bt.nn_search(&search_point, 5);
// knn will be top 5 nearest points in the tree,
// assuming at least 5 points in the ball tree.
```

### Custom Types UNTESTED
Straight vectors not enough for you? Store more data with custom types!

Define a custom type for the ball tree and implement the `Baller` and `Clone` traits:
```rust
// define a custom struct
#[derive(Clone)]
struct CustomType {
    id: u64,
    name: String,
    vector: Vec<f32>
}

// pub trait Baller {
//     fn metric(&self, &Self) -> f32;
//     fn midpoint(&self, &f32, &Self, &f32) -> Self;
// }

impl Baller for CustomType {
    // Distance. Generally how far apart the center of the balls are away from each-other
    fn metric(&self, other: &CustomType) -> f32 {
        use ball_tree::vector_math::*;
        
        distance(&self.vector, &other.vector) // euclidean distance
    }

    // Midpoint. The (halfway point) from ball 1 to ball 2.
    // Could be based on euclidean distance, or an english dictionary lookup!
    fn midpoint(&self, self_rad: &f32, other: &CustomType, other_rad: &f32) -> CustomType {
        use ball_tree::vector_math::*;
        // compute the spacial midpoint using geometry
        let span = subtract_vec(&self.vector, &other.vector);
        let mag = magnitude(&span);
        let unit_vec = divide_scal(&span, &mag);
        let p1 = add_vec(&self.vector, &multiply_scal(&unit_vec, &self_rad));
        let p2 = subtract_vec(&other.vector, &multiply_scal(&unit_vec, &other_rad));
        CustomType {
            id: 0,
            name: "",
            vector: midpoint(&p1, &p2)
        }
    }
}
```

Use the ball tree:
```rust
let bt: BallTree<CustomType> = BallTree::new();
let bt_updated = bt.push(&CustomType {
    id: 1,
    name: "The Origin",
    vector: vec![0., 0., 0., 0., 0.,]
});
```

## Testing
For sanity check tests run `cargo test`.

## Benchmarking
For benchmarking results run `cargo bench`.
Note that for each test, the average random tree generation time must be subtracted.
e.g. the true result of `ball_tree_push_18x10_bench` is actually (the result of `ball_tree_push_18x10_bench`) - (the result of `random_benchmark_tree_18x10`).

### Results
If you just want to see some quick results and graphs, check out these:

***TODO***

## In Progress:
- benchmarking results
- prevent stack overflows on large trees
- load performance (optimal tree construction)
- delete nodes from ball tree
- allow tree reshape for better search performance
