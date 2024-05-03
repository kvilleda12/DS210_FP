//breadth first searhc algorithm which finds the usual distnaces between pairs of nodes 
//also try to find average distances 
//also find the shortest path for a couple hundred nodes and compare what makes them so far form each other 

use std::io::{self, BufRead, BufReader};
use std::collections::VecDeque;
use rand::thread_rng;
use rand::seq::SliceRandom;


pub fn shortest_path(graph: &[Vec<usize>], start: usize, end: usize) -> Option<usize> {
    // finds the shortest path from node a to node b
    let mut queue = VecDeque::new(); //initializes queue 
    let mut distances = vec![None; graph.len()]; //to store the distance from the start to every other node
    distances[start] = Some(0);
    queue.push_back(start);
    
    while let Some(current) = queue.pop_front() {
        if current == end {
            return distances[current]; //if the destination node return distance to that node
        }
        for &neighbor in &graph[current] { //checks neighboring nodes distances
            if distances[neighbor].is_none() {
                distances[neighbor] = Some(distances[current].unwrap() + 1); //sets neighbors distance to (current node distance+1) and queues this neighbor
                queue.push_back(neighbor);
            }
        }
    }
    None
}

// Function to compute the average degrees of separation using BFS
pub fn average_degrees_of_separation(graph: &[Vec<usize>], sample_size: usize) -> Option<f64> {
    let mut rng = thread_rng(); //to radnomly select nodes from the grpah
    let node_index: Vec<usize> = (0..graph.len()).collect(); //create vector of all possible node pairings in a graph
    let sampled_indexes = node_index.choose_multiple(&mut rng, sample_size);
    
    let mut total_distance = 0;
    let mut paths_counted = 0;
    let mut counter = 0;

    println!("Running on {} nodes", sampled_indexes.len());
    for &start in sampled_indexes { //finds distance of all nodes from each node and calcualtes this average
        counter += 1;
        if counter % 100 == 0 {
            println!("Starting at node {}", start);
        }
        let (distance_sum, count) = bfs_average_path_length(graph, start);
        total_distance += distance_sum;
        paths_counted += count;
    }
    //calculates the average
    if paths_counted > 0 {
        Some(total_distance as f64 / paths_counted as f64)
    } else {
        None
    }
}


pub fn bfs_average_path_length(graph: &[Vec<usize>], start: usize) -> (usize, usize) {

    let mut queue = VecDeque::new(); //creates queue vector
    let mut distances = vec![None; graph.len()];

    distances[start] = Some(0);
    queue.push_back(start);

    let mut distance_sum = 0;
    let mut count = 0; //count of number of nodes reachable from the node

    while let Some(current) = queue.pop_front() { //checks the node in the queue
        if let Some(d) = distances[current] {   
            distance_sum += d; //adds the distance if the current node has a distance added 
            count += 1; 
            //checks the neighbors of the current node
            for &neighbor in &graph[current] {
                if distances[neighbor].is_none() {
                    distances[neighbor] = Some(d + 1); //neighbor node stiance to the current node is the current distance + 1
                    queue.push_back(neighbor); //puts neighbor back into the queue 
                }
            }
        }
    }
   
    (distance_sum, count - 1) //avoid self looping
}

pub fn reachable_within_steps(graph: &[Vec<usize>], start: usize, max_steps: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut distances = vec![None; graph.len()];
    distances[start] = Some(0);
    queue.push_back((start, 0));

    let mut reachable_count = 0;

    while let Some((current, dist)) = queue.pop_front() { //current node and its distance are processed
        if dist < max_steps {
            for &neighbor in &graph[current] {
                if distances[neighbor].is_none() {
                    distances[neighbor] = Some(dist + 1); //neighbor distance is node distance + 1
                    queue.push_back((neighbor, dist + 1)); 
                    reachable_count += 1; //number of unqiue nodes reached
                }
            }
        }
    }

    reachable_count
}









