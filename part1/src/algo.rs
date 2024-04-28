//breadth first searhc algorithm which finds the usual distnaces between pairs of nodes 
//also try to find average distances 
//also find the shortest path for a couple hundred nodes and compare what makes them so far form each other 

use std::io::{self, BufRead, BufReader};
use std::collections::VecDeque;
 

pub fn shortest_path(graph: &[Vec<usize>], start: usize, end: usize) -> Option<usize> {
    let mut queue = VecDeque::new(); //initializes queue 
    let mut distances = vec![None; graph.len()]; //to store the distance from the start to every other node
    distances[start] = Some(0);
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        if current == end {
            return distances[current];
        }
        for &neighbor in &graph[current] { 
            if distances[neighbor].is_none() {
                distances[neighbor] = Some(distances[current].unwrap() + 1); //sets neighbors distance to (current node distance+1) and queues this neighbor
                queue.push_back(neighbor);
            }
        }
    }
    None
}
