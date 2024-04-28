mod algo;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use algo::shortest_path;

fn read_graph(filename: &str) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts = line.split('\t').collect::<Vec<&str>>();
        let current_node = parts[0].parse::<usize>()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?; //to handle IO:Error
        let destination_node = parts[1].parse::<usize>()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        let max_index = usize::max(current_node, destination_node); //finds highest index
        if max_index >= edges.len() { //compares to see if max index is at least as learge as the adjacency list
            edges.resize(max_index + 1, Vec::new()); 
        }
        edges[current_node].push(destination_node); //makes adjacency list 
    }
    Ok(edges)
}
fn main() -> io::Result<()> {
    let graph = read_graph("amazon0312.txt")?;
    let start_node = 0; // Example start node
    let end_node = 6; // Example end node

    if let Some(degree) = shortest_path(&graph, start_node, end_node) {
        println!("Degrees of separation between {} and {} is {}", start_node, end_node, degree);
    } else {
        println!("No path found between {} and {}", start_node, end_node);
    }

    Ok(())
}
