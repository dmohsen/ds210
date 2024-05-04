use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;
use std::thread;


#[derive(Debug, Deserialize, Clone)]
struct Song {
    position: usize,
    artist_name: String,
    song_name: String,
    days: u32,
    top_10_x_times: Option<f32>, 
    peak_position: u32,
    peak_position_x_times: String, 
    peak_streams: u32,
    total_streams: u64, 
}

type Vertex = usize;
type Weight = isize; 
type AdjacencyLists = Vec<Vec<(Vertex, Weight)>>;

struct Graph {
    vertices: Vec<Song>, 
    adjacency_list: AdjacencyLists, 
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: vec![],
            adjacency_list: vec![], 
        }
    }

    fn add_vertex(&mut self, song: Song) -> usize {
        let index = self.vertices.len();
        self.vertices.push(song);
        self.adjacency_list.push(Vec::new());
        index
    }


    fn add_weighted_edge_by_features(&mut self, src: usize, dest: usize) {
        if src != dest {
            let peak_position_diff = (self.vertices[src].peak_position as f64 - self.vertices[dest].peak_position as f64).abs();
            let weight = (0.5 * peak_position_diff / 10.0 * 1000.0) as isize;
            self.adjacency_list[src].push((dest, weight));
            self.adjacency_list[dest].push((src, weight));
        }
    }
    
    

    fn build_from_songs(songs: Vec<Song>) -> Self {
        let mut graph = Self::new();
        let mut artist_map: HashMap<String, Vec<usize>> = HashMap::new();
    
        for song in &songs {
            let index = graph.add_vertex(song.clone());
        }
    
        for i in 0..songs.len() {
            for j in 0..songs.len() {
                if i != j {
                    graph.add_weighted_edge_by_features(i, j);
                }
            }
        }
    
        graph
    }


    fn dijkstra(&self, start_vertex: usize) -> Vec<f32> {
        let mut distances = vec![f32::MAX; self.vertices.len()];
        let mut heap = BinaryHeap::new();

        distances[start_vertex] = 0.0;
        heap.push(Reverse((0, start_vertex)));

        while let Some(Reverse((current_distance, u))) = heap.pop() {
            if (current_distance as f32) > distances[u] {
                continue;
            }

            for &(v, weight) in &self.adjacency_list[u] {
                let distance = current_distance + weight;
                if (distance as f32) < distances[v] {
                    distances[v] = distance as f32;
                    heap.push(Reverse((distance, v)));
                }
            }
        }

        distances.iter().map(|&d| d / 1000.0).collect()
    }


    fn closeness_centrality(&self) -> HashMap<usize, (String, f32)> {
        let mut centrality_scores = HashMap::new();
        for (i, _) in self.vertices.iter().enumerate() {
            let distances = self.dijkstra(i);
            let sum_distances: f32 = distances.iter().filter(|&&d| d != f32::MAX).sum();
            let closeness = if sum_distances > 0.0 { 1.0 / sum_distances } else { 0.0 };
            centrality_scores.insert(i, (self.vertices[i].song_name.clone(), closeness));
        }
        centrality_scores
    }

    fn print_most_central_for_depth(&self) {
        let closeness_scores = self.closeness_centrality();
        let mut max_closeness_per_depth: HashMap<usize, (String, f32)> = HashMap::new();

        for (vertex, (_song_name, closeness)) in closeness_scores {
            if let Some((_, max_closeness)) = max_closeness_per_depth.get(&vertex) {
                if closeness > *max_closeness {
                    max_closeness_per_depth.insert(vertex, (self.vertices[vertex].song_name.clone(), closeness));
                }
            } else {
                max_closeness_per_depth.insert(vertex, (self.vertices[vertex].song_name.clone(), closeness));
            }
        }

        for depth in 1..=6 {
            if let Some((song_name, closeness)) = max_closeness_per_depth.get(&depth) {
                println!("Depth {}: Song: {}, Closeness: {}", depth, song_name, closeness);
            }
        }
    }
    
}

fn load_songs_from_csv(file_path: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut songs = vec![];

    for result in rdr.deserialize() {
        let song: Song = result?;
        songs.push(song);
    }

    Ok(songs)
}

fn main() {
    let songs = load_songs_from_csv("realSpotify_final_dataset.csv").expect("Failed to load songs");
    let graph = Graph::build_from_songs(songs);
    graph.print_most_central_for_depth();
}