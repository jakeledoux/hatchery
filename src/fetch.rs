use rustfm::{user::recent_tracks, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FetchError;

impl Error for FetchError {}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to fetch scrobbles.")
    }
}

struct Pages {
    current: usize,
    total: usize,
}

impl Default for Pages {
    fn default() -> Self {
        Pages {
            current: 1,
            total: 0,
        }
    }
}

/// Re-implementation of `rustfm::user::recent_tracks::Track` to allow for serde serialization.
#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    pub artist: String,
    pub name: String,
    pub album: String,
    pub album_artist: String,
    pub timestamp: String,
}
// TODO: Fuck rustfm, they don't have full attributes. I'm gonna have to write a custom Last.fm
// interface.

impl From<recent_tracks::Track> for Track {
    fn from(track: recent_tracks::Track) -> Self {
        Track {
            artist: track.artist.name,
            name: track.name,
            album: track.album.name,
            album_artist: track.
        }
    }
}

pub fn fetch_scrobbles(client: &mut Client, username: &str) -> Result<Vec<Track>, FetchError> {
    let mut scrobbles: Vec<Track> = Vec::new();
    let mut pages = Pages::default();
    loop {
        println!(
            "Fetching page {} of {}.",
            pages.current,
            if pages.total > 0 {
                pages.total.to_string()
            } else {
                "?".to_string()
            }
        );
        if let Ok(recent_tracks) = client
            .recent_tracks(username)
            .with_limit(200)
            .with_page(pages.current)
            .send()
        {
            // Update pages
            if pages.total == 0 {
                pages.total = recent_tracks
                    .attrs
                    .total_pages
                    .parse()
                    .expect("Failed to parse total page count.");
            }
            pages.current += 1;

            // Parse scrobbles
            scrobbles.extend(recent_tracks.tracks.into_iter().map(Track::from))
        } else {
            eprintln!("Oh no! Failed to fetch recent tracks.");
            return Err(FetchError);
        }
        if pages.current == 5 {
            return Ok(scrobbles);
        }
    }
}
