# `ball-tree`

Ball Tree implementation in Rust.

Ball trees are binary-tree like data structures useful for storing and searching for hyperdimensional (of more than 3 dimensions) vectors. The ball tree is inherently imbalanced so lookup is likely to be higher than `log(n)` in the majority of cases. Ball trees are fantastic for quickly solving nearest-neighbor and k-means clustering for larger datasets, and are very appliccable to many machine learning applications in production as it allows for latent feature space 