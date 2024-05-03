use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use std::collections::{HashMap, VecDeque, BinaryHeap};
use std::cmp::Reverse;

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
type Weight = isize; // Using isize for weights to utilize integer comparisons
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

    fn add_weighted_edge(&mut self, src: usize, dest: usize) {
        let src_pos = self.vertices[src].position;
        let dest_pos = self.vertices[dest].position;
        let weight = ((src_pos as isize - dest_pos as isize).abs() * 1000) as isize; 

        if src != dest {
            self.adjacency_list[src].push((dest, weight));
            self.adjacency_list[dest].push((src, weight));
        }
    }
    
    fn build_from_songs(songs: Vec<Song>) -> Self {
        let mut graph = Self::new();
        let mut artist_map: HashMap<String, Vec<usize>> = HashMap::new();

        for song in songs {
            let index = graph.add_vertex(song.clone());
            artist_map.entry(song.artist_name.clone()).or_default().push(index);
        }

        for indices in artist_map.values() {
            for &i in indices {
                for &j in indices {
                    if i != j {
                        graph.add_weighted_edge(i, j);
                    }
                }
            }
        }

        graph
    }

    fn print_graph(&self) {
        for (i, edges) in self.adjacency_list.iter().enumerate() {
            let artist_name = &self.vertices[i].artist_name;
            println!("{}: {} Connections", artist_name, edges.len());
            for &(j, weight) in edges {
                let connected_song = &self.vertices[j].song_name;
                println!("  - Connects to {} with weight {:.3}", connected_song, weight as f32 / 1000.0);
            }
        }
    }

    fn bfs(&self, start_vertex: usize) -> Vec<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.vertices.len()];
        let mut result = Vec::new();

        visited[start_vertex] = true;
        queue.push_back(start_vertex);

        while let Some(current) = queue.pop_front() {
            result.push(current);
            for &(neighbor, _) in &self.adjacency_list[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        result
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
    let songs = load_songs_from_csv("Spotify_final_dataset.csv").expect("Failed to load songs");
    let graph = Graph::build_from_songs(songs);
    graph.print_graph();
}
