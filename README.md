# Amazon Product Co-Purchasing Network Analysis

## Project Overview

This project aims to analyze the Amazon Product Co-Purchasing Network to understand the connectivity and relationship between products based on co-purchase patterns. Specifically, we calculate the average, maximum, and median shortest path lengths between product nodes to gauge how closely products are interconnected. This analysis provides insights that can help optimize Amazon's recommendation systems and improve product visibility.

## Goals

- Calculate the average shortest path length to understand the overall network connectivity.
- Identify extreme path lengths to discover niche products or categories.
- Enhance recommendation algorithms based on the network structure of product co-purchases.

## Dataset

The dataset used in this project consists of approximately 403,394 nodes and 3,387,388 directed edges, reflecting products and their co-purchasing relationships as of June 1, 2003.

**Data source:** [SNAP: Amazon co-purchasing network](https://snap.stanford.edu/data/amazon0601.html)

## Setup Instructions

Follow these steps to set up and run the analysis:

### Prerequisites

- Rust Programming Language
- Cargo (Rust's package manager and build tool)

### Installation

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/amazon-network-analysis.git
    cd amazon-network-analysis
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

### Running the Analysis

To execute the analysis and view the output:

```bash
cargo run --release
