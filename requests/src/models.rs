use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub name: String,
    pub external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub name: String,
    pub artists: Vec<Artist>,
    pub external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub name: String,
    pub href: String,
    pub popularity: u32,
    pub album: Album,
    pub external_urls: ExternalUrls,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    tracks: Items<Track>,
}

impl Display for APIResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "API Response:")?;
        for track in &self.tracks.items {
            writeln!(f, "Track name: {}", track.name)?;
            writeln!(f, "Album name: {}", track.album.name)?;
            let artists = track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>();
            writeln!(f, "Artists: {}", artists)?;
            writeln!(f, "-----")?;
        }
        Ok(())
    }
}
