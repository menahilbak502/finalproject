//metrics.rs
use crate::graph_ops::{bfs_shortest_paths, DiGraph}; // Import necessary functions and types from graph_ops.
use crate::stat_util::{median, standard_deviation};   // Import statistical functions from stat_util module.
use std::collections::HashMap;                        // Import HashMap for storing centrality values.
use rand::seq::SliceRandom;                           // Import random sampling utilities from the rand crate.

/// Computes closeness centrality for each node in a sample of the graph.
pub fn calculate_closeness_centrality(graph: &DiGraph<u32, ()>, sample_size: usize) -> HashMap<usize, f64> {
    let mut closeness_centralities = HashMap::new();   // HashMap to store closeness centrality values.
    let mut rng = rand::thread_rng();                  // Random number generator for sampling nodes.
    let node_indices: Vec<usize> = graph.node_indices().map(|n| n.index()).collect(); // Collect all node indices.
    let sampled_nodes = node_indices.choose_multiple(&mut rng, sample_size).cloned().collect::<Vec<_>>(); // Randomly sample nodes.

    // Iterate over each sampled node to calculate closeness centrality.
    for &node in &sampled_nodes {
        let distances = bfs_shortest_paths(graph, node); // Calculate shortest paths from the node to all others.
        let total_distance: u32 = distances.iter().filter_map(|d| *d).sum(); // Sum all distances for the node.
        let closeness = if total_distance > 0 {          // Check if total distance is greater than 0.
            (graph.node_count() - 1) as f64 / total_distance as f64 // Compute closeness centrality.
        } else {
            0.0                                         // Return 0.0 for isolated nodes.
        };
        closeness_centralities.insert(node, closeness); // Store the centrality value in the HashMap.
    }

    closeness_centralities                             // Return the map with centrality values.
}

/// Calculates path metrics for a sample of nodes within the graph.
pub fn calculate_path_metrics(graph: &DiGraph<u32, ()>, sample_size: usize) -> (f64, f64, f64, f64) {
    let mut rng = rand::thread_rng();                  // Random number generator for sampling nodes.
    let node_indices: Vec<usize> = graph.node_indices().map(|n| n.index()).collect(); // Collect all node indices.
    let sampled_nodes = node_indices.choose_multiple(&mut rng, sample_size).cloned().collect::<Vec<_>>(); // Randomly sample nodes.

    let mut all_distances = Vec::new();                // Vector to store distances from sampled nodes.
    // Calculate distances from each sampled node to all others.
    for &node in &sampled_nodes {
        let distances = bfs_shortest_paths(graph, node); // Get shortest paths from each node.
        all_distances.extend(distances.iter().filter_map(|&d| d).map(|d| d as f64)); // Store valid distances as f64.
    }

    let max_distance = all_distances.iter().cloned().fold(f64::NEG_INFINITY, f64::max); // Calculate max distance.
    let mean_distance = all_distances.iter().sum::<f64>() / all_distances.len() as f64; // Calculate mean distance.
    let median_distance = median(&mut all_distances.clone());                          // Calculate median distance.
    let std_deviation = standard_deviation(&all_distances, mean_distance);             // Calculate standard deviation.

    (mean_distance, max_distance, median_distance, std_deviation)                      // Return the calculated metrics as a tuple.
}