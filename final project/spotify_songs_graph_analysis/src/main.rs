use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
struct Song {
    position: u32,
    artist_name: String,
    song_name: String,
    days: u32,
    top_10_x_times: Option<f32>, 
    peak_position: u32,
    peak_position_x_times: String, 
    peak_streams: u32,
    total_streams: u64, 
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
    match load_songs_from_csv("Spotify_final_dataset.csv") {
        Ok(songs) => {
            println!("Loaded {} songs.", songs.len());
            for song in songs.iter().take(10) {
                println!("{:?}", song);
            }
        },
        Err(e) => eprintln!("Error loading songs: {}", e),
    }
}

