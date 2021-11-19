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

    // Get loved tracks
    let mut loved_tracks: Vec<LovedTrack> = Vec::new();
    log::info!("Fetching loved tracks...");
    if let Ok(fetched_tracks) = client.loved_tracks(&opt.username) {
        loved_tracks.extend(fetched_tracks);
        log::info!("Done!");
    } else {
        log::error!("Failed to fetch loved tracks");
    }

    // Get scrobbles
    let mut friends: Vec<Friend> = Vec::new();
    log::info!("Fetching friends...");
    if let Ok(fetched_friends) = client.friends(&opt.username) {
        friends.extend(fetched_friends);
        log::info!("Done!");
    } else {
        log::error!("Failed to fetch friends");
    }

    // Get friends
    let mut scrobbles: Vec<Track> = Vec::new();
    log::info!("Fetching recent tracks...");
    if let Ok(fetched_tracks) = client.recent_tracks(&opt.username) {
        scrobbles.extend(fetched_tracks);
        log::info!("Done!");
    } else {
        log::error!("Failed to fetch recent tracks");
    }

    // Create database and insert data

    log::info!("Opening database...");
    if let Ok(mut conn) = open_db(DatabaseLocation::Disk(opt.database)) {
        log::info!("Creating tables...");
        if create_tables(&mut conn).is_ok() {
            // Begin inserting data

            if !loved_tracks.is_empty() {
                log::info!("Inserting loved tracks...");
                if insert_loved_tracks(&mut conn, loved_tracks).is_ok() {
                    log::info!("Done!");
                } else {
                    log::error!("Failed to insert loved tracks. Continuing...");
                }
            } else {
                log::warn!("No loved tracks fetched. Skipping.");
            }

            if !friends.is_empty() {
                log::info!("Inserting friends...");
                if insert_friends(&mut conn, friends).is_ok() {
                    log::info!("Done!");
                } else {
                    log::error!("Failed to insert friends. Continuing...");
                }
            } else {
                log::warn!("No friends fetched. Skipping.");
            }

            if !scrobbles.is_empty() {
                log::info!("Inserting scrobbles...");
                if insert_scrobbles(&mut conn, scrobbles).is_ok() {
                    log::info!("Done!");
                } else {
                    log::error!("Failed to insert scrobbles.");
                }
            } else {
                log::warn!("No scrobbles fetched. Skipping.");
            }

            log::info!("Done!");
            close_db(conn).expect("Failed to close db???????");
        } else {
            log::error!("Failed to create tables.")
        }
    } else {
        log::error!("Failed to open DB. Check the provided path.")
    }
}
