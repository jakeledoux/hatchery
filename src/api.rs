use anyhow::anyhow;
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
    #[serde(rename(deserialize = "#text"))]
    pub pretty_string: String,
    #[serde_as(
        deserialize_as = "TimestampSeconds<String, Strict>",
        serialize_as = "TimestampSeconds<i64, Strict>"
    )]
    #[serde(rename(deserialize = "uts", serialize = "timestamp"))]
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
    #[serde(
        rename(deserialize = "#text"),
        with = "string_empty_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub url: Option<String>,
    pub size: ImageSize,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Artist {
    #[serde(rename(deserialize = "#text"))]
    pub name: String,
    #[serde(with = "string_empty_as_none", skip_serializing_if = "Option::is_none")]
    pub mbid: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Album {
    #[serde(rename(deserialize = "#text"))]
    pub name: String,
    #[serde(with = "string_empty_as_none", skip_serializing_if = "Option::is_none")]
    pub mbid: Option<String>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct TrackAttributes {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename(deserialize = "nowplaying"))]
    pub now_playing: bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Track {
    #[serde(rename(deserialize = "@attr"), skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TrackAttributes>,
    pub artist: Artist,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<Album>,
    pub name: String,
    pub image: Vec<Image>, // TODO: Skip images that don't contain URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    pub url: String,
    #[serde(with = "string_empty_as_none", skip_serializing_if = "Option::is_none")]
    pub mbid: Option<String>,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RequestAttributes {
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub page: usize,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename(deserialize = "perPage"))]
    pub per_page: usize,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    pub total: usize,
    #[serde_as(deserialize_as = "DisplayFromStr")]
    #[serde(rename(deserialize = "totalPages"))]
    pub total_pages: usize,
    #[serde(rename(deserialize = "user"))]
    pub username: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RecentTracks {
    #[serde(rename(deserialize = "@attr"))]
    pub attributes: RequestAttributes,
    #[serde(rename(deserialize = "track"))]
    pub tracks: Vec<Track>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RecentTracksResponse {
    #[serde(rename(deserialize = "recenttracks"))]
    pub recent_tracks: RecentTracks,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LovedArtist {
    pub name: String,
    #[serde(with = "string_empty_as_none", skip_serializing_if = "Option::is_none")]
    pub mbid: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LovedTrack {
    #[serde(rename(deserialize = "@attr"), skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TrackAttributes>,
    pub artist: LovedArtist,
    pub name: String,
    pub image: Vec<Image>, // TODO: Skip images that don't contain URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    pub url: String,
    #[serde(with = "string_empty_as_none", skip_serializing_if = "Option::is_none")]
    pub mbid: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LovedTracks {
    #[serde(rename(deserialize = "@attr"))]
    pub attributes: RequestAttributes,
    #[serde(rename(deserialize = "track"))]
    pub tracks: Vec<LovedTrack>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LovedTracksResponse {
    #[serde(rename(deserialize = "lovedtracks"))]
    pub loved_tracks: LovedTracks,
}

#[serde_as]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RegisterDate {
    #[serde(rename(deserialize = "#text"))]
    pub pretty_string: String,
    #[serde_as(
        deserialize_as = "TimestampSeconds<String, Strict>",
        serialize_as = "TimestampSeconds<i64, Strict>"
    )]
    #[serde(rename(deserialize = "unixtime", serialize = "timestamp"))]
    pub datetime: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Friend {
    pub name: String,
    pub image: Vec<Image>, // TODO: Skip images that don't contain URLs
    pub country: String,
    pub url: String,
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub subscriber: bool,
    #[serde(
        rename(deserialize = "realname"),
        with = "string_empty_as_none",
        skip_serializing_if = "Option::is_none"
    )]
    pub real_name: Option<String>,
    pub registered: RegisterDate,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Friends {
    #[serde(rename(deserialize = "@attr"))]
    pub attributes: RequestAttributes,
    #[serde(rename(deserialize = "user"))]
    pub friends: Vec<Friend>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FriendsResponse {
    friends: Friends,
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

    pub fn authenticate(&mut self, username: &str, password: &str) -> anyhow::Result<()> {
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
            return Err(anyhow!(LastFMError::AuthError));
        }

        Ok(())
    }

    pub fn recent_tracks(&mut self, username: &str) -> anyhow::Result<Vec<Track>> {
        let mut tracks: Vec<Track> = Vec::new();
        let mut page = 1;
        let mut total_pages = 0;
        let mut failures = 0;

        loop {
            let mut success = false;
            log::info!(
                "Requesting page {} of {}",
                page,
                match total_pages {
                    0 => "?".to_string(),
                    _ => total_pages.to_string(),
                }
            );
            if let Ok(resp) = self.get(
                "user.getRecentTracks",
                vec![
                    ("user".to_string(), username.to_string()),
                    ("limit".to_string(), "200".to_string()),
                    ("page".to_string(), page.to_string()),
                ],
            ) {
                if let Ok(response) = resp.json::<RecentTracksResponse>() {
                    tracks.extend(response.recent_tracks.tracks);

                    success = true;
                    failures = 0;
                    page += 1;

                    let new_total_pages = response.recent_tracks.attributes.total_pages;
                    match new_total_pages.cmp(&total_pages) {
                        std::cmp::Ordering::Greater => {
                            total_pages = new_total_pages;
                        }
                        std::cmp::Ordering::Less => {
                            log::warn!(
                                "Total pages shrunk from {} to {}. Ignoring",
                                total_pages,
                                new_total_pages
                            );
                        }
                        _ => {}
                    }

                    if page > total_pages {
                        // TODO: remove true
                        break Ok(tracks);
                    }
                }
            }
            if !success {
                failures += 1;
                if failures < 3 {
                    log::warn!("Failed to get page. Retrying...");
                } else {
                    log::error!("Max retries reached. Aborting.");
                    break Err(anyhow!(LastFMError::RequestError));
                }
            }
        }
    }

    pub fn loved_tracks(&mut self, username: &str) -> anyhow::Result<Vec<LovedTrack>> {
        let mut tracks: Vec<LovedTrack> = Vec::new();
        let mut page = 1;
        let mut total_pages = 0;
        let mut failures = 0;

        loop {
            let mut success = false;
            log::info!(
                "Requesting page {} of {}",
                page,
                match total_pages {
                    0 => "?".to_string(),
                    _ => total_pages.to_string(),
                }
            );
            if let Ok(resp) = self.get(
                "user.getLovedTracks",
                vec![
                    ("user".to_string(), username.to_string()),
                    ("limit".to_string(), "200".to_string()),
                    ("page".to_string(), page.to_string()),
                ],
            ) {
                let response = resp.json::<LovedTracksResponse>();
                if let Ok(response) = response {
                    tracks.extend(response.loved_tracks.tracks);

                    success = true;
                    failures = 0;
                    page += 1;

                    let new_total_pages = response.loved_tracks.attributes.total_pages;
                    match new_total_pages.cmp(&total_pages) {
                        std::cmp::Ordering::Greater => {
                            total_pages = new_total_pages;
                        }
                        std::cmp::Ordering::Less => {
                            log::warn!(
                                "Total pages shrunk from {} to {}. Ignoring...",
                                total_pages,
                                new_total_pages
                            );
                        }
                        _ => {}
                    }

                    if page > total_pages {
                        // TODO: remove true
                        break Ok(tracks);
                    }
                }
            }
            if !success {
                failures += 1;
                if failures < 3 {
                    log::warn!("Failed to get page. Retrying...");
                } else {
                    log::error!("Max retries reached. Aborting.");
                    break Err(anyhow!(LastFMError::RequestError));
                }
            }
        }
    }

    pub fn friends(&mut self, username: &str) -> anyhow::Result<Vec<Friend>> {
        let mut friends: Vec<Friend> = Vec::new();
        let mut page = 1;
        let mut total_pages = 0;
        let mut failures = 0;

        loop {
            let mut success = false;
            log::info!(
                "Requesting page {} of {}",
                page,
                match total_pages {
                    0 => "?".to_string(),
                    _ => total_pages.to_string(),
                }
            );
            if let Ok(resp) = self.get(
                "user.getFriends",
                vec![
                    ("user".to_string(), username.to_string()),
                    ("limit".to_string(), "50".to_string()),
                    ("page".to_string(), page.to_string()),
                ],
            ) {
                if let Ok(response) = resp.json::<FriendsResponse>() {
                    friends.extend(response.friends.friends);

                    success = true;
                    failures = 0;
                    page += 1;

                    let new_total_pages = response.friends.attributes.total_pages;
                    match new_total_pages.cmp(&total_pages) {
                        std::cmp::Ordering::Greater => {
                            total_pages = new_total_pages;
                        }
                        std::cmp::Ordering::Less => {
                            log::warn!(
                                "Total pages shrunk from {} to {}. Ignoring",
                                total_pages,
                                new_total_pages
                            );
                        }
                        _ => {}
                    }

                    if page > total_pages {
                        // TODO: remove true
                        break Ok(friends);
                    }
                }
            }
            if !success {
                failures += 1;
                if failures < 3 {
                    log::warn!("Failed to get page. Retrying...");
                } else {
                    log::error!("Max retries reached. Aborting.");
                    break Err(anyhow!(LastFMError::RequestError));
                }
            }
        }
    }
}
