# `ball-tree`

Ball Tree implementation in Rust.

Ball Trees are Binary-Tree-like data structures useful for storing and searching for hyperdimensional (of more than 3 dimensions) vectors. The Ball Tree is inherently imbalanced, so lookup is likely to be **higher** than `log(n)` in the majority of cases. Ball Trees are fantastic for quickly solving nearest-neighbor and k-means clustering problems, for larger datasets. They're also very applicable to many machine learning applications in production, as they allow for latent feature space.
