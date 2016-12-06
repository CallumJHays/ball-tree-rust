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

use ball_tree::{BallTree, Ball};
```

Create a new ball tree:

In this example, the ball tree key is of type `Vec<f32>` and leaf node values are `String`s
```rust
let mut bt: BallTree<Vec<f32>, String> = BallTree::new()
```

Insert Items into ball tree:
```rust
let new_node = Ball::new(vec![1., 2., 3...], "Some data can go here")
bt.push(new_node);
```

Search for k nearest neighbors in a ball tree.
```rust
let search_key: Vec<f32> = vec![1., 2., 3...];
let knn: Vec<Ball::Leaf<Vec<f32>, String> = bt.nn_search(&search_point, 5);
// knn will be top 5 nearest points in the tree,
// assuming at least 5 points in the ball tree.
for neighbor in knn.into_iter() {
    print!("{:?} -> {:?}", neighbor.key, neighbor.val);
}
```

### Custom Types
By default and in the examples above, the ball tree structure is based of of vectors for keys and the euclidean distance between those vectors. However, you can use any data type as a key as long as you have a valid implementation of the difference function between keys and a midpoint function that can calculate a midpoint between two given keys. By devising these custom implementations you can also 

Define a custom key for the ball tree and implement the `HasMeasurableDiff` trait (or impl the trait on a primitive type like `String`:
```rust
// define a custom key - val pair
type CustomKey = Vec<f32>;
struct CustomVal { id: u64, name: String }

// pub trait HasMeasurableDiff {
//     fn difference(&self, other: &Self) -> f32;
//     fn midpoint(&self, other: &Self, self_rad: f32, other_rad: f32) -> Self;
// }

## Testing
For sanity check tests run `cargo test`.

## Benchmarking
For benchmarking results run `cargo bench`. 
Quite a lot of time (hours) is required to run the entire benchmarking suite.
If you're only interested in a few things, please comment out what you aren not interested in.
The naming scheme for benchmarks is as follows:

`<benchmark_name>_<ball_tree_size>_<vector_size>_bench`
Where `ball_tree_size` is expressed as actual_size = 2^`ball_tree_size`
and `vector_size` is expressed as vector_dimensions = 2^`vector_size`

Note that for each test, the average time taken to clone the tree must be subtracted.
e.g. the true result of `ball_tree_push_18x10_bench` is actually:

(the result of `ball_tree_push_18x10_bench`) - (the result of `clone_tree_18x10_bench`).

### Results
If you just want to see some quick results and graphs, check out these:

***TODO***

## In Progress:
- benchmarking results
- prevent stack overflows on large trees
- load performance (optimal tree construction)
- delete nodes from ball tree
- allow tree reshape for better search performance
