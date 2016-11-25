use std::io;

fn main() {
    println!("Find the number of nodes in a binary-like tree");
    let mut input = String::new();

    println!("Please input the spread parameter (default 2):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let spread: u32 = input.trim().parse().expect("Please type a number!");

    let mut input = String::new();
    println!("Please input the amount of leaf nodes:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nodes: u64 = input.trim().parse().expect("Please type a number!");

    let total = calc_total_nodes(spread, nodes, 0);
    println!("The total number of nodes in the tree will be {}", total);
}

fn calc_total_nodes(spread: u32, nodes: u64, count: u64) -> u64 {
    if nodes == 0 {
        count
    } else {
        calc_total_nodes(spread, nodes / 2, count + nodes)
    }
}
