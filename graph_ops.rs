// graph_ops.rs
pub use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use flate2::read::GzDecoder;


/// Constructs the graph from a gzipped file containing edges.
pub fn construct_graph(file_path: &str) -> DiGraph<u32, ()> {
   let file = File::open(file_path).expect("Failed to open file");
   let decoder = GzDecoder::new(file);
   let reader = BufReader::new(decoder);
   let mut graph = DiGraph::<u32, ()>::new();
   let mut index_map = HashMap::new();


   for line in reader.lines() {
       let line = line.expect("Failed to read line");
       if line.starts_with('#') {
           continue; // Skip comment lines
       }
       let parts: Vec<&str> = line.split_whitespace().collect();
       let from_node: u32 = parts[0].parse().expect("Failed to parse from node");
       let to_node: u32 = parts[1].parse().expect("Failed to parse to node");


       let from_index = *index_map.entry(from_node).or_insert_with(|| graph.add_node(from_node).index());
       let to_index = *index_map.entry(to_node).or_insert_with(|| graph.add_node(to_node).index());


       graph.add_edge(NodeIndex::new(from_index), NodeIndex::new(to_index), ());
   }


   graph
}


/// Runs a BFS from a given start node and returns the distances to all reachable nodes.
pub fn bfs_shortest_paths(graph: &DiGraph<u32, ()>, start_index: usize) -> Vec<Option<u32>> {
   let mut distances = vec![None; graph.node_count()];
   let mut visit_queue = VecDeque::new();


   let start_node = NodeIndex::new(start_index);
   distances[start_index] = Some(0);
   visit_queue.push_back(start_node);


   while let Some(node) = visit_queue.pop_front() {
       let current_distance = distances[node.index()].unwrap();


       for neighbor in graph.neighbors(node) {
           if distances[neighbor.index()].is_none() {
               distances[neighbor.index()] = Some(current_distance + 1);
               visit_queue.push_back(neighbor);
           }
       }
   }


   distances
}


/// Analyzes and prints the degree distribution of the graph.
pub fn analyze_degree_distribution(graph: &DiGraph<u32, ()>) {
   let mut degree_count = HashMap::new();
   for node_id in graph.node_indices() {
       let degree = graph.neighbors_directed(node_id, petgraph::Direction::Outgoing). count();  // For directed graphs
       *degree_count.entry(degree).or_insert(0) += 1;
   }
   println!("Degree Distribution: {:?}", degree_count);
}
