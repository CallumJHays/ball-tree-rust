# Rust Ball-Tree implementation

***WARNING: Incomplete. Please do not use yet.***

Immutable Ball Tree implementation in Rust.

Ball Trees are Binary-Tree-like data structures useful for storing and searching for hyperdimensional (of more than 3 dimensions) vectors. The Ball Tree is inherently imbalanced, so lookup is likely to be **higher** than `log(n)` in the majority of cases. Ball Trees are fantastic for quickly solving nearest-neighbor and k-means clustering problems for larger datasets. They're also very applicable to many machine learning applications in production, as they allow for efficient latent feature space search, useful in recommendation systems.

## Usage
Add crate to cargo.toml
```toml
[dependencies]
ball-tree = { git = "https://github.com/callumjhays/ball-tree-rust }
```

Import crate for use:
```rust
extern crate ball_tree;

use ball_tree::BallTree;
```

Create a new ball tree:
```rust
let bt = BallTree::new()
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

## In Progress:
- custom metric other than euclidean distance
- k-d construction (load tree from collection)
- tree flatten to collection (for saving)
- delete vectors from ball tree
- implement as generic
- allow tree reshape for better search performance
