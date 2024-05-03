mod algo;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use algo::shortest_path;
use algo::average_degrees_of_separation;
use rand::thread_rng;
use rand::seq::SliceRandom;
use algo::reachable_within_steps;

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

    // Run shortest path from the 1st to the 50,000 node
    if graph.len() > 50000 {
        if let Some(distance) = shortest_path(&graph, 0, 49999) { 
            println!("Shortest path from node 1 to node 50,000 is {} steps.", distance);
        } else {
            println!("No path found from node 1 to node 50,000.");
        }
    } 
    // Calculate percentage of nodes that are reachble within a sample size of 10,000
    let sample_size = 50000;
    let node_indices: Vec<usize> = (0..graph.len()).collect();
    let mut rng = thread_rng();
    let sampled_indices = node_indices.choose_multiple(&mut rng, sample_size);

    let mut total_percent_reachable = 0.0;
    for &start in sampled_indices {
        let reachable = reachable_within_steps(&graph, start, 10);
        let percent_reachable = (reachable as f64 / (graph.len() - 1) as f64) * 100.0;
        total_percent_reachable += percent_reachable;
    }


    let average_percent_reachable = total_percent_reachable / sample_size as f64;
    println!("Average percentage of nodes reachable within 10 steps (sampled): {:.2}%", average_percent_reachable);

    if let Some(average) = average_degrees_of_separation(&graph, sample_size) {
        println!("Estimated average degrees of separation: {:.2}", average);
    } else {
        println!("Could not calculate the average degrees of separation.");
    }

    Ok(())
}

//for test methods
fn create_test_graph() -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; 169]; 

    let edges = vec![
        (0, 1), (0, 2), (0, 3), (0, 4), (0, 5),
        (1, 0), (1, 2), (1, 13), (1, 14), (1, 15),
        (2, 0), (2, 1), (2, 4), (2, 5), (2, 16),
        (3, 70), (3, 71), (3, 72), (3, 73), (3, 74),
        (4, 7), (4, 17), (4, 18), (4, 19), (4, 20),
        (5, 6), (5, 7), (5, 8), (5, 9), (5, 10),
        (6, 5), (6, 7), (6, 8), (6, 9), (6, 21),
        (7, 5), (7, 8), (7, 9), (7, 11), (7, 12),
        (8, 6), (8, 11), (8, 22), (8, 23), (8, 24),
        (9, 5), (9, 6), (9, 7), (9, 8), (9, 11),
        (10, 5), (10, 7), (10, 166), (10, 167), (10, 168),
    ];

    for (from, to) in edges {
        graph[from].push(to);
    }

    graph
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shortest_path() {
        let graph = create_test_graph();
        let path_length = algo::shortest_path(&graph, 0, 15); 
        assert_eq!(path_length, Some(2)); 
    }
    #[test]
    fn test_average_degrees_of_separation() {
        let graph = create_test_graph();
        let avg_degrees = algo::average_degrees_of_separation(&graph, 10);
        assert!(avg_degrees.is_some());
        println!("Average degrees of separation: {:?}", avg_degrees);
    }
    #[test]
    fn test_reachable_within_steps() {
        let graph = create_test_graph();
        let reachable = algo::reachable_within_steps(&graph, 0, 2);
        assert_eq!(reachable, 23);
    }
}
