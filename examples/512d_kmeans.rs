extern crate ball_tree;
extern crate rand;

mod helper;

use std::time::Instant;
use helper::*;

fn main() {
    // load 1'000'000 points into the tree
    println!("generating ball tree");
    let now = Instant::now();
    rand_balltree(20, 3);
    println!("time taken to generate ball tree: {}s, {}ns", now.elapsed().as_secs(), now.elapsed().subsec_nanos());
}

