// main.rs
mod graph_ops;
mod metrics;
mod stat_util;
use std::env;



fn main() {
    println!("Current directory: {:?}", env::current_dir().unwrap());

    let graph = graph_ops::construct_graph("../amazon0601.txt.gz");
   println!("Number of nodes: {}", graph.node_count());
   println!("Number of edges: {}", graph.edge_count());


   graph_ops::analyze_degree_distribution(&graph);


   let closeness_centrality = metrics::calculate_closeness_centrality(&graph, 25);
   println!("Closeness centrality for sampled nodes: {:?}", closeness_centrality);


   let (average_distance, max_distance, median_distance, std_deviation) = metrics::calculate_path_metrics(&graph, 100);
   println!("Average shortest path length from sample of 100 nodes: {}", average_distance);
   println!("Maximum shortest path length from sample of 100 nodes: {}", max_distance);
   println!("Median shortest path length from sample of 100 nodes: {}", median_distance);
   println!("Standard deviation of shortest path length from sample of 100 nodes: {}", std_deviation);
}
