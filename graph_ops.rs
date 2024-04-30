//graph_ops.rs
pub use petgraph::graph::{DiGraph, NodeIndex};  // Re-export DiGraph and NodeIndex from petgraph for direct use in other modules.
use std::collections::{HashMap, VecDeque};     // Import HashMap for node indexing and VecDeque for queue management in BFS.
use std::fs::File;                             // Import File for file operations.
use std::io::{BufRead, BufReader};             // Import BufRead and BufReader for efficient buffered reading.
use flate2::read::GzDecoder;                   // Import GzDecoder to handle gzip compressed files.

/// Constructs a directed graph from a gzipped file containing edge definitions.
pub fn construct_graph(file_path: &str) -> DiGraph<u32, ()> {
    let file = File::open(file_path).expect("Failed to open file"); // Open the specified gzip file, panicking if it fails.
    let decoder = GzDecoder::new(file); // Create a GzDecoder to decompress the gzip file.
    let reader = BufReader::new(decoder); // Wrap the decoder in a BufReader for efficient reading.
    let mut graph = DiGraph::<u32, ()>::new(); // Initialize an empty directed graph.
    let mut index_map = HashMap::new(); // Create a hashmap to track the mapping of node values to graph node indices.

    // Read lines from the file, skipping comments and empty lines.
    for line in reader.lines() {
        let line = line.expect("Failed to read line"); // Read each line, panicking if an error occurs.
        if line.starts_with('#') {
            continue; // Skip comment lines that start with '#'.
        }
        let parts: Vec<&str> = line.split_whitespace().collect(); // Split the line into parts using whitespace.
        let from_node: u32 = parts[0].parse().expect("Failed to parse from node"); // Parse the 'from' node ID.
        let to_node: u32 = parts[1].parse().expect("Failed to parse to node"); // Parse the 'to' node ID.

        // Use or create node indices for 'from' and 'to' nodes, adding them to the graph if they are new.
        let from_index = *index_map.entry(from_node).or_insert_with(|| graph.add_node(from_node).index());
        let to_index = *index_map.entry(to_node).or_insert_with(|| graph.add_node(to_node).index());

        graph.add_edge(NodeIndex::new(from_index), NodeIndex::new(to_index), ()); // Add an edge between the 'from' and 'to' nodes.
    }

    graph // Return the constructed graph.
}

/// Performs a breadth-first search (BFS) to find shortest paths from a start node in the graph.
pub fn bfs_shortest_paths(graph: &DiGraph<u32, ()>, start_index: usize) -> Vec<Option<u32>> {
    let mut distances = vec![None; graph.node_count()]; // Initialize distances vector with None for each node.
    let mut visit_queue = VecDeque::new(); // Create a queue for nodes to visit.

    let start_node = NodeIndex::new(start_index); // Get the start node index.
    distances[start_index] = Some(0); // Set the distance to the start node as 0.
    visit_queue.push_back(start_node); // Add the start node to the queue.

    // Process the queue until empty.
    while let Some(node) = visit_queue.pop_front() {
        let current_distance = distances[node.index()].unwrap(); // Get the current node's distance.

        // Visit each neighbor of the current node.
        for neighbor in graph.neighbors(node) {
            if distances[neighbor.index()].is_none() { // Check if the neighbor has not been visited.
                distances[neighbor.index()] = Some(current_distance + 1); // Set the neighbor's distance.
                visit_queue.push_back(neighbor); // Add the neighbor to the queue for further exploration.
            }
        }
    }

    distances // Return the vector of distances.
}

/// Analyzes and returns the degree distribution of the graph.
pub fn analyze_degree_distribution(graph: &DiGraph<u32, ()>) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new(); // Create a hashmap to count degrees.
    // Iterate over all nodes to compute their out-degrees.
    for node_id in graph.node_indices() {
        let degree = graph.neighbors_directed(node_id, petgraph::Direction::Outgoing).count(); // Count outgoing edges for each node.
        *degree_count.entry(degree).or_insert(0) += 1; // Increment the count for this degree in the map.
    }
    degree_count // Return the degree distribution map.
}