use serde::Deserialize;
use csv::ReaderBuilder;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Song {
    pub artist_name: String,
    pub song_name: String,
    pub days: u32,
    pub peak_position: u32,
    pub total_streams: u64, 
}

pub fn load_songs_from_csv(file_path: &str) -> Result<Vec<Song>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut songs = vec![];

    for result in rdr.deserialize() {
        let song: Song = result?;
        songs.push(song);
    }

    Ok(songs)
}

