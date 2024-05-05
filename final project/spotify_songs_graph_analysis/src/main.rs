mod song;
mod graph;

use crate::song::{load_songs_from_csv, Song};
use crate::graph::Graph;



fn main() {
    let songs = load_songs_from_csv("Spotify_final_dataset.csv").expect("Failed to load songs");

    let graph = Graph::build_from_songs(songs);

    graph.print_most_central_for_depth();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, seq::SliceRandom};

    fn get_random_subset(file_path: &str, subset_size: usize) -> Vec<Song> {
        let songs = load_songs_from_csv(file_path).unwrap();
        let mut rng = thread_rng();
        let mut shuffled_songs = songs.clone();
        shuffled_songs.shuffle(&mut rng);
        shuffled_songs.into_iter().take(subset_size).collect()
    }

    #[test]
    fn test_graph_construction_with_random_subset() {
        let song_subset = get_random_subset("Spotify_final_dataset.csv", 1000);
        let graph = Graph::build_from_songs(song_subset);
        assert!(!graph.vertices.is_empty());
        assert_eq!(graph.vertices.len(), 1000);
        assert_ne!(graph.adjacency_list[0].len(), 0);
    }

    #[test]
    fn test_dijkstra_algorithm() {
        let song_subset = get_random_subset("Spotify_final_dataset.csv", 1000);
        let graph = Graph::build_from_songs(song_subset);
        let distances = graph.dijkstra(0);
        assert!(distances[1] > 0.0);
        assert_ne!(distances[1], f32::MAX);
    }

    #[test]
    fn test_closeness_centrality() {
        let song_subset = get_random_subset("Spotify_final_dataset.csv", 1000);
        let graph = Graph::build_from_songs(song_subset);
        let centrality_scores = graph.closeness_centrality();
        assert!(centrality_scores.iter().any(|(_, &(_, _, closeness))| closeness > 0.0));
    }

    #[test]
    fn test_graph_connectivity() {
        let song_subset = get_random_subset("Spotify_final_dataset.csv", 1000);
        let graph = Graph::build_from_songs(song_subset);
        assert!(graph.is_connected());
    }
}
