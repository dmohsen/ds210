mod graph;          
mod simulation;    

use graph::parse_graph;
use simulation::{simulate_random_walks, compute_pagerank};

fn main() {
    let path = "pagerank_data.txt";
    let graph = parse_graph(path);

    let vertex_count = graph.len();
    let total_walks = 100 * vertex_count;
    let visit_counts = simulate_random_walks(&graph, vertex_count);
    let rankings = compute_pagerank(&visit_counts, total_walks);

    let top_five = rankings.iter().take(5);
    for (vertex, pagerank) in top_five {
        println!("vertex {}: approximate PageRank {:.4}", vertex, pagerank);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_random_walks() {
        let graph = vec![vec![1], vec![2], vec![0]];
        let vertex_count = graph.len();
        
        let visit_counts = simulate_random_walks(&graph, vertex_count);
        
        for count in visit_counts {
            assert!(count > 0, "Each node should have a nonzero visit count");
        }
    }

    #[test]
    fn test_compute_pagerank() {
        let visit_counts = vec![100, 200, 300]; 
        let total_walks = 600; 
        
        let rankings = compute_pagerank(&visit_counts, total_walks);
        
        let expected_rankings = vec![
            (2, 0.5), 
            (1, 1.0 / 3.0), 
            (0, 1.0 / 6.0) 
        ];

        assert_eq!(rankings, expected_rankings);
    }
}
