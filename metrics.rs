// metrics.rs
use crate::graph_ops::{bfs_shortest_paths, DiGraph};
use crate::stat_util::{median, standard_deviation};
use std::collections::HashMap;
use rand::seq::SliceRandom;


/// Computes the closeness centrality for each node in the graph.
pub fn calculate_closeness_centrality(graph: &DiGraph<u32, ()>, sample_size: usize) -> HashMap<usize, f64> {
   let mut closeness_centralities = HashMap::new();
   let mut rng = rand::thread_rng();
   let node_indices: Vec<usize> = graph.node_indices().map(|n| n.index()).collect();
   let sampled_nodes = node_indices.choose_multiple(&mut rng, sample_size).cloned().collect::<Vec<_>>();


   for &node in &sampled_nodes {
       let distances = bfs_shortest_paths(graph, node);
       let total_distance: u32 = distances.iter().filter_map(|d| *d).sum();
       let closeness = if total_distance > 0 {
           (graph.node_count() - 1) as f64 / total_distance as f64
       } else {
           0.0
       };
       closeness_centralities.insert(node, closeness);
   }


   closeness_centralities
}


/// Enhanced version that calculates average, max, median, and standard deviation of shortest paths.
pub fn calculate_path_metrics(graph: &DiGraph<u32, ()>, sample_size: usize) -> (f64, f64, f64, f64) {
   let mut rng = rand::thread_rng();
   let node_indices: Vec<usize> = graph.node_indices().map(|n| n.index()).collect();
   let sampled_nodes = node_indices.choose_multiple(&mut rng, sample_size).cloned().collect::<Vec<_>>();


   let mut all_distances = Vec::new();
   for &node in &sampled_nodes {
       let distances = bfs_shortest_paths(graph, node);
       all_distances.extend(distances.iter().filter_map(|&d| d).map(|d| d as f64));
   }


   let max_distance = all_distances.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
   let mean_distance = all_distances.iter().sum::<f64>() / all_distances.len() as f64;
   let median_distance = median(&mut all_distances.clone());
   let std_deviation = standard_deviation(&all_distances, mean_distance);


   (mean_distance, max_distance, median_distance, std_deviation)
}