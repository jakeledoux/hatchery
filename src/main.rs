pub mod api;
pub mod sql;

use api::*;
use clap::Parser;
use sql::*;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Jake Ledoux (contactjakeledoux@gmail.com)")]
struct Opts {
    #[clap(env = "LASTFM_USERNAME")]
    username: String,
    #[clap(long, default_value = "lastfm.db")]
    database: String,
    #[clap(long, env = "LASTFM_API_KEY")]
    api_key: String,
    #[clap(long, env = "LASTFM_API_SECRET")]
    api_secret: String,
}

fn main() {
    // Init logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Load environment variables
    log::debug!("Loading environment variables");
    dotenv::dotenv().ok();

    // Parse program options
    log::debug!("Parsing Clap options");
    let opt: Opts = Opts::parse();

    // Create last.fm api client
    let mut client = LastFM::new(&opt.api_key, &opt.api_secret);

    let mut tracks: Vec<Track> = Vec::new();
    log::info!("Fetching recent tracks...");
    if let Ok(fetched_tracks) = client.recent_tracks(&opt.username) {
        tracks.extend(fetched_tracks);
        log::info!("Done!");
    } else {
        log::error!("Failed to fetch tracks");
    }

    let mut friends: Vec<Friend> = Vec::new();
    log::info!("Fetching friends...");
    if let Ok(fetched_friends) = client.friends(&opt.username) {
        friends.extend(fetched_friends);
        log::info!("Done!");
    } else {
        log::error!("Failed to fetch friends");
    }

    log::info!("Opening database...");
    let mut conn = open_db(DatabaseLocation::Disk(opt.database)).expect("Failed to open database.");

    log::info!("Creating tables...");
    create_tables(&mut conn).expect("Failed to create tables???");

    if !tracks.is_empty() {
        log::info!("Inserting scrobbles...");
        insert_scrobbles(&mut conn, tracks).expect("Failed to insert scrobbles.");
    } else {
        log::warn!("Skipping inserting scrobbles.");
    }

    if !friends.is_empty() {
        log::info!("Inserting friends...");
        insert_friends(&mut conn, friends).expect("Failed to insert scrobbles.");
    } else {
        log::warn!("Skipping inserting scrobbles.");
    }

    log::info!("Done!");
    close_db(conn).expect("Failed to close db???????");
}
