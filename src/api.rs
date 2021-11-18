use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_bool_from_anything;
use serde_with::{
    formats::Strict, rust::string_empty_as_none, serde_as, DisplayFromStr, TimestampSeconds,
};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LastFMError {
    AuthError,
    RequestError,
}

impl Error for LastFMError {}

impl fmt::Display for LastFMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LastFMError::AuthError => {
                write!(f, "Failed to authenticate with Last.fm.")
            }
            LastFMError::RequestError => {
                write!(f, "Failed to fetch scrobbles.")
            }
        }
    }
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Date {
    #[serde(rename = "#text")]
    pub pretty_string: String,
    #[serde_as(as = "TimestampSeconds<String, Strict>")]
    #[serde(rename = "uts")]
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "#text")]
    pub url: String,
    pub size: ImageSize,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Artist {
    #[serde(rename = "#text")]
    pub name: String,
    #[serde(with = "string_empty_as_none")]
    pub mbid: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Album {
    #[serde(rename = "#text")]
    pub name: String,
    #[serde(with = "string_empty_as_none")]
    pub mbid: Option<String>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TrackAttributes {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "nowplaying")]
    pub now_playing: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Track {
    #[serde(rename = "@attr")]
    pub attributes: Option<TrackAttributes>,
    pub artist: Artist,
    pub album: Album,
    pub name: String,
    pub image: Vec<Image>,
    pub date: Option<Date>,
    pub url: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub streamable: bool,
    #[serde(with = "string_empty_as_none")]
    pub mbid: Option<String>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RecentTracksAttributes {
    #[serde_as(as = "DisplayFromStr")]
    pub page: usize,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "perPage")]
    pub per_page: usize,
    #[serde_as(as = "DisplayFromStr")]
    pub total: usize,
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
    #[serde(rename = "user")]
    pub username: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RecentTracks {
    #[serde(rename = "@attr")]
    pub attributes: RecentTracksAttributes,
    #[serde(rename = "track")]
    pub tracks: Vec<Track>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RecentTracksResponse {
    #[serde(rename = "recenttracks")]
    pub recent_tracks: RecentTracks,
}

pub struct LastFM {
    http_client: reqwest::blocking::Client,
    endpoint: String,
    api_key: String,
    api_secret: String,
    session_key: Option<String>,
}

impl LastFM {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        LastFM {
            http_client: reqwest::blocking::Client::new(),
            endpoint: "http://ws.audioscrobbler.com/2.0".to_string(),
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            session_key: None,
        }
    }

    fn get_signature(&self, mut query: Vec<(String, String)>) -> String {
        query.sort_by_key(|e| e.0.clone());

        let mut signature = String::new();
        for (key, value) in query {
            signature.push_str(&(key + &value));
        }
        signature.push_str(&self.api_secret);

        let digest = md5::compute(signature);
        let signature = format!("{:x}", digest);
        signature
    }

    fn build_query(&self, method: &str, mut query: Vec<(String, String)>) -> Vec<(String, String)> {
        query.push(("method".to_string(), method.to_string()));
        query.push(("api_key".to_string(), self.api_key.clone()));
        query.push(("api_sig".to_string(), self.get_signature(query.clone())));
        query.push(("format".to_string(), "json".to_string()));
        query
    }

    fn get(
        &self,
        method: &str,
        mut query: Vec<(String, String)>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        query = self.build_query(method, query);
        let req = self
            .http_client
            .get(format!("{}/", self.endpoint))
            .query(&query);
        let req = req.build()?;
        self.http_client.execute(req)
    }

    fn post(
        &self,
        method: &str,
        mut query: Vec<(String, String)>,
    ) -> Result<reqwest::blocking::Response, reqwest::Error> {
        query = self.build_query(method, query);
        let req = self
            .http_client
            .post(format!("{}/", self.endpoint))
            .form(&query);
        let req = req.build()?;
        self.http_client.execute(req)
    }

    pub fn authenticate(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let resp = self.post(
            "auth.getMobileSession",
            vec![
                ("username".to_string(), username.to_string()),
                ("password".to_string(), password.to_string()),
            ],
        )?;
        let auth_response: serde_json::Value = resp.json()?;

        self.session_key = auth_response["session"]["key"].as_str().map(String::from);

        if self.session_key.is_none() {
            return Err(Box::new(LastFMError::AuthError));
        }

        Ok(())
    }

    pub fn recent_tracks(&mut self, username: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        let page = 1;
        let resp = self.get(
            "user.getRecentTracks",
            vec![
                ("user".to_string(), username.to_string()),
                ("limit".to_string(), "200".to_string()),
                ("page".to_string(), page.to_string()),
            ],
        )?;

        let recent_tracks: serde_json::Value = resp.json()?;
        Ok(recent_tracks)
    }
}
