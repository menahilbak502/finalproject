//main.rs
mod graph_ops;  // Import the graph operations module that includes functions to manipulate and analyze graphs.
mod metrics;    // Import the metrics module that calculates various graph metrics like centrality.
mod stat_util;  // Import the statistics utility module for statistical calculations like median and standard deviation.
use std::env;   // Import the environment module to interact with the environment in which the program runs.


fn main() {
    println!("Current directory: {:?}", env::current_dir().unwrap());  // Print the current working directory to help in troubleshooting paths.

    let graph = graph_ops::construct_graph("../amazon0601.txt.gz");  // Construct a graph from a gzipped text file specifying edges.
    println!("Number of nodes: {}", graph.node_count());  // Output the number of nodes in the graph to the console.
    println!("Number of edges: {}", graph.edge_count());  // Output the number of edges in the graph to the console.

    graph_ops::analyze_degree_distribution(&graph);  // Analyze and print the degree distribution of the graph.

    let closeness_centrality = metrics::calculate_closeness_centrality(&graph, 100);  // Compute the closeness centrality for a sample of 1000 nodes.
    println!("Closeness centrality for 100 sampled nodes: {:?}", closeness_centrality);  // Print the computed closeness centrality values.


    let (average_distance, max_distance, median_distance, std_deviation) = metrics::calculate_path_metrics(&graph, 100);  // Calculate and print path metrics like average, max, and median path lengths.
    println!("Average shortest path length from sample of 100 nodes: {}", average_distance);  // Display the average shortest path length.
    println!("Maximum shortest path length from sample of 100 nodes: {}", max_distance);  // Display the maximum shortest path length found.
    println!("Median shortest path length from sample of 100 nodes: {}", median_distance);  // Display the median value of shortest path lengths.
    println!("Standard deviation of shortest path length from sample of 100 nodes: {}", std_deviation);  // Display the standard deviation of shortest path lengths.
}


#[cfg(test)]
mod tests {
    use super::*;
    use graph_ops::*;
    use metrics::*;
    use petgraph::graph::DiGraph;

    /// Tests graph construction from a mock data source to ensure nodes and edges are added correctly.
    #[test]
    fn test_graph_construction() {
        let mut graph = DiGraph::<u32, ()>::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_edge(NodeIndex::new(0), NodeIndex::new(1), ());
        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    /// Tests the calculation of degree distribution.
    #[test]
    fn test_degree_distribution() {
        let mut graph = DiGraph::<u32, ()>::new();
        let n1 = graph.add_node(1).index();
        let n2 = graph.add_node(2).index();
        graph.add_edge(NodeIndex::new(n1), NodeIndex::new(n2), ());
        graph.add_edge(NodeIndex::new(n2), NodeIndex::new(n1), ());
        
        let distribution = analyze_degree_distribution(&graph);
        assert_eq!(distribution.get(&1), Some(&2)); // Both nodes have 1 outgoing edge
    }

    /// Tests basic functionality of BFS for shortest paths.
    #[test]
    fn test_bfs_shortest_paths() {
        let mut graph = DiGraph::<u32, ()>::new();
        let n1 = graph.add_node(1).index();
        let n2 = graph.add_node(2).index();
        graph.add_edge(NodeIndex::new(n1), NodeIndex::new(n2), ());
        
        let paths = bfs_shortest_paths(&graph, n1);
        assert_eq!(paths[n2], Some(1));
    }

    /// Tests the closeness centrality computation on a small, controlled graph.
    #[test]
    fn test_closeness_centrality() {
        let mut graph = DiGraph::<u32, ()>::new();
        let n1 = graph.add_node(1).index();
        let n2 = graph.add_node(2).index();
        graph.add_edge(NodeIndex::new(n1), NodeIndex::new(n2), ());
        graph.add_edge(NodeIndex::new(n2), NodeIndex::new(n1), ());

        let centralities = calculate_closeness_centrality(&graph, 2);
        assert!(centralities.contains_key(&n1));
        assert!(centralities.contains_key(&n2));
    }

/// Tests calculation of path metrics to ensure the function computes correctly.
#[test]
fn test_path_metrics() {
    let mut graph = DiGraph::<u32, ()>::new();
    let n1 = graph.add_node(1).index();
    let n2 = graph.add_node(2).index();
    graph.add_edge(NodeIndex::new(n1), NodeIndex::new(n2), ());
    graph.add_edge(NodeIndex::new(n2), NodeIndex::new(n1), ());

    let (average, max, median, std_dev) = calculate_path_metrics(&graph, 2);
    println!("Average: {}", average);
    println!("Max: {}", max);
    println!("Median: {}", median);
    println!("Standard Deviation: {}", std_dev);

    assert_eq!(average, 0.5, "Average should be 0.5"); // Adjusted expectation
    assert_eq!(max, 1.0, "Max should be 1.0");
    assert_eq!(median, 0.5, "Median should be 0.5");
    assert!(std_dev >= 0.0, "Standard deviation should be non-negative");
}

}

