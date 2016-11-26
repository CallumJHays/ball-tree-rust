# `ball-tree`

***WARNING: Incomplete. Please do not use yet.***

Immutable Ball Tree implementation in Rust.

Ball Trees are Binary-Tree-like data structures useful for storing and searching for hyperdimensional (of more than 3 dimensions) vectors. The Ball Tree is inherently imbalanced, so lookup is likely to be **higher** than `log(n)` in the majority of cases. Ball Trees are fantastic for quickly solving nearest-neighbor and k-means clustering problems for larger datasets. They're also very applicable to many machine learning applications in production, as they allow for efficient latent feature space search, useful in recommendation systems.

## Usage
Add crate to cargo.toml
```
[dependencies]
ball-tree = { git = "https://github.com/callumjhays/ball-tree-rust }
```

Import crate for use:
```
extern crate ball_tree;

use ball_tree::BallTree;
```

Create a new ball tree:
```
let bt = BallTree::new()
```

Insert Items into ball tree:
```
let vector: Vec<f32> = vec![1., 2., 3...];
let updated_tree = bt.push(&vector);
```

## In Progress:
- k Nearest Neighbor Search
- k-d construction (load tree from collection)
- delete vectors from ball tree
- implement as generic
- custom metric other than euclidean distance
