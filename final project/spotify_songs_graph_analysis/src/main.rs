use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
use std::collections::HashSet;

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
type ListOfEdges = Vec<(Vertex, Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

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
        self.adjacency_list.push(vec![]);
        index
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        if !self.adjacency_list[src].contains(&dest) {
            self.adjacency_list[src].push(dest);
            self.adjacency_list[dest].push(src); 
        }
    }
    
    fn build_from_songs(songs: Vec<Song>) -> Self {
        let mut graph = Self::new();
        let mut artist_map: HashMap<String, Vec<usize>> = HashMap::new();

        for song in songs.into_iter() {
            let index = graph.add_vertex(song.clone());
            artist_map.entry(song.artist_name).or_default().push(index);
        }

        for indices in artist_map.values() {
            for &i in indices {
                for &j in indices {
                    if i != j {
                        graph.add_edge(i, j);
                    }
                }
            }
        }

        graph
    }
    
    fn print_graph(&self) {
        let mut artist_map: HashMap<&String, HashSet<&String>> = HashMap::new();

        for (i, edges) in self.adjacency_list.iter().enumerate() {
            let artist_name = &self.vertices[i].artist_name;
            if !edges.is_empty() { 
                artist_map.entry(artist_name).or_default();
                for &index in edges {
                    let connected_song = &self.vertices[index].song_name;
                    let connected_artist = &self.vertices[index].artist_name;
                    if artist_name == connected_artist {
                        artist_map.get_mut(artist_name).unwrap().insert(connected_song);
                    }
                }
            }
        }

        for (artist, songs) in artist_map {
            println!("{}: {} Songs", artist, songs.len());
            for song in songs {
                println!("  - {} ", song);
            }
            println!(); 
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
    let songs = match load_songs_from_csv("Spotify_final_dataset.csv") {
        Ok(songs) => {
            println!("Loaded {} songs.", songs.len());
            for song in songs.iter().take(10) {
                println!("{:?}", song);
            }
            songs  
        },
        Err(e) => {
            eprintln!("Error loading songs: {}", e);
            return; 
        },
    };

    let mut graph = Graph::build_from_songs(songs);
    graph.print_graph();
}

