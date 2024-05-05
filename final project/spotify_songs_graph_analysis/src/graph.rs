use super::song::Song;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use ordered_float::OrderedFloat;

pub type Vertex = usize;
pub type Weight = isize; 
pub type AdjacencyLists = Vec<Vec<(Vertex, Weight)>>;


pub struct Graph {
    pub vertices: Vec<Song>, 
    pub adjacency_list: AdjacencyLists, 
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: vec![],
            adjacency_list: vec![], 
        }
    }

    pub fn add_vertex(&mut self, song: Song) -> usize {
        let index = self.vertices.len();
        self.vertices.push(song);
        self.adjacency_list.push(Vec::new());
        index
    }

    pub fn add_weighted_edge_by_features(&mut self, src: usize, dest: usize) {
        let song_a = &self.vertices[src];
        let song_b = &self.vertices[dest];
    
        if src != dest {
            let peak_position_diff = (song_a.peak_position as isize - song_b.peak_position as isize).abs();
            let stream_diff = (song_a.total_streams as isize - song_b.total_streams as isize).abs();
            let days_diff = (song_a.days as isize - song_b.days as isize).abs();
    
            let weight = 1000.0 / (1.0 + 0.05 * peak_position_diff as f32 + 0.01 * stream_diff as f32 / 1000000.0 + 0.1 * days_diff as f32 / 10.0);
            
            self.adjacency_list[src].push((dest, weight as isize));
            self.adjacency_list[dest].push((src, weight as isize));
        }
    }
    
    pub fn build_from_songs(songs: Vec<Song>) -> Self {
        let mut graph = Self::new();
    
        for song in &songs {
            graph.add_vertex(song.clone());
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

    pub fn dijkstra(&self, start_vertex: usize) -> Vec<f32> {
        let mut distances = vec![f32::MAX; self.vertices.len()];
        let mut heap = BinaryHeap::new();
    
        distances[start_vertex] = 0.0;
        heap.push(Reverse((OrderedFloat(0.0), start_vertex)));
    
        while let Some(Reverse((OrderedFloat(current_distance), u))) = heap.pop() {
            if current_distance > distances[u] {
                continue;
            }
    
            for &(v, weight) in &self.adjacency_list[u] {
                let distance = current_distance + weight as f32;
                if distance < distances[v] {
                    distances[v] = distance;
                    heap.push(Reverse((OrderedFloat(distance), v)));
                }
            }
        }
    
        distances
    }    
    
    pub fn closeness_centrality(&self) -> HashMap<usize, (String, String, f32)> {
        let mut centrality_scores = HashMap::new();

        for (i, song) in self.vertices.iter().enumerate() {
            let distances = self.dijkstra(i);
            let sum_distances: f32 = distances.iter().filter(|&&d| d < f32::MAX).sum();
            let reachable_nodes = distances.iter().filter(|&&d| d < f32::MAX).count() as f32;
            let closeness = if reachable_nodes > 0.0 { reachable_nodes / sum_distances } else { 0.0 }; 
            let closeness = closeness * 1000.0;
            centrality_scores.insert(i, (song.song_name.clone(), song.artist_name.clone(), closeness));
        }
        centrality_scores
    }
    
    
    pub fn print_most_central_for_depth(&self) {
        let closeness_scores = self.closeness_centrality();
        let mut sorted_scores: Vec<_> = closeness_scores.iter().collect();
        sorted_scores.sort_by(|a, b| b.1 .2.partial_cmp(&a.1 .2).unwrap());
    
        for (i, adj_list) in self.adjacency_list.iter().enumerate() {
            println!("Vertex {} has {} edges", i, adj_list.len());
        }
    
        for (depth, (_, (song_name, artist_name, closeness))) in sorted_scores.iter().enumerate().take(6) {
            println!("Depth {}: Song: {}, Artist: {}, Closeness: {:.2}", depth + 1, song_name, artist_name, closeness);
        }
    }


    pub fn is_connected(&self) -> bool {
        let mut visited = vec![false; self.vertices.len()];
        let mut stack = vec![0]; 
    
        while let Some(node) = stack.pop() {
            if !visited[node] {
                visited[node] = true;
                for &(neighbour, _) in &self.adjacency_list[node] {
                    if !visited[neighbour] {
                        stack.push(neighbour);
                    }
                }
            }
        }
    
        visited.iter().all(|&v| v)
    }
    
}