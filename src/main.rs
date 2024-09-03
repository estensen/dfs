fn main() {
    let root = dfs::generate_tree();

    // Pretty print tree
    println!("Tree structure:");
    root.pretty_print();

    // Perform DFS and collect visited nodes
    let mut visited = Vec::new();
    root.dfs(&mut |value| visited.push(*value));

    println!("DFS order: {:?}", visited);
}
