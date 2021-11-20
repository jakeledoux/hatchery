pub mod api;
mod serialize;
mod sql;

use api::*;
use clap::{ArgEnum, Parser};
use sql::*;

// TODO: CSV serialization
#[derive(ArgEnum, Clone)]
enum ExportFormat {
    Json,
    Sql,
}

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Jake Ledoux (contactjakeledoux@gmail.com)")]
struct Opts {
    #[clap(env = "LASTFM_USERNAME")]
    username: String,
    #[clap(arg_enum, short = 'f', long, default_value = "json")]
    format: ExportFormat,
    #[clap(long, env = "LASTFM_API_KEY")]
    api_key: String,
    #[clap(long, env = "LASTFM_API_SECRET")]
    api_secret: String,
}

fn make_filename(template: &str) -> String {
    let now = chrono::Local::now();
    now.format(template).to_string()
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

    // Export data
    match opt.format {
        ExportFormat::Json => {
            log::info!("Writing JSON...");
            if !loved_tracks.is_empty() {
                let loved_tracks_filename = make_filename("hatchery-%Y-%m-%d-loved_tracks.json");
                log::debug!("Inserting loved tracks...");
                if serialize::write_json(loved_tracks_filename, &loved_tracks).is_ok() {
                    log::debug!("Done!");
                } else {
                    log::error!("Failed to write loved tracks. Continuing...");
                }
            } else {
                log::warn!("No loved tracks fetched. Skipping.");
            }

            if !friends.is_empty() {
                let friends_filename = make_filename("hatchery-%Y-%m-%d-friends.json");
                log::debug!("Inserting friends...");
                if serialize::write_json(friends_filename, &friends).is_ok() {
                    log::debug!("Done!");
                } else {
                    log::error!("Failed to write friends. Continuing...");
                }
            } else {
                log::warn!("No friends fetched. Skipping.");
            }

            if !scrobbles.is_empty() {
                let scrobbles_filename = make_filename("hatchery-%Y-%m-%d-scrobbles.json");
                log::debug!("Inserting scrobbles...");
                if serialize::write_json(scrobbles_filename, &scrobbles).is_ok() {
                    log::debug!("Done!");
                } else {
                    log::error!("Failed to write scrobbles.");
                }
            } else {
                log::warn!("No scrobbles fetched. Skipping.");
            }
        }
        ExportFormat::Sql => {
            let db_filename = make_filename("hatchery-%Y-%m-%d.db");

            log::info!("Writing database...");
            if let Ok(mut conn) = open_db(&db_filename) {
                log::debug!("Creating tables...");
                if create_tables(&mut conn).is_ok() {
                    // Begin inserting data

                    if !loved_tracks.is_empty() {
                        log::debug!("Inserting loved tracks...");
                        if insert_loved_tracks(&mut conn, loved_tracks).is_ok() {
                            log::debug!("Done!");
                        } else {
                            log::error!("Failed to insert loved tracks. Continuing...");
                        }
                    } else {
                        log::warn!("No loved tracks fetched. Skipping.");
                    }

                    if !friends.is_empty() {
                        log::debug!("Inserting friends...");
                        if insert_friends(&mut conn, friends).is_ok() {
                            log::debug!("Done!");
                        } else {
                            log::error!("Failed to insert friends. Continuing...");
                        }
                    } else {
                        log::warn!("No friends fetched. Skipping.");
                    }

                    if !scrobbles.is_empty() {
                        log::debug!("Inserting scrobbles...");
                        if insert_scrobbles(&mut conn, scrobbles).is_ok() {
                            log::debug!("Done!");
                        } else {
                            log::error!("Failed to insert scrobbles.");
                        }
                    } else {
                        log::warn!("No scrobbles fetched. Skipping.");
                    }
                    close_db(conn).expect("Failed to close db???????");
                } else {
                    log::error!("Failed to create tables.")
                }
            } else {
                log::error!("Failed to open DB. Check the provided path.")
            }
            log::info!("Finished writing database.");
        }
    }
}
