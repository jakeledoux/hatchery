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
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Parse program options
    // let opt: Opts = Opts::parse();

    // DEBUG: Load cached response from JSON
    if let Ok(file) = std::fs::File::open("response.json") {
        let response: RecentTracksResponse =
            serde_json::from_reader(file).expect("failed to read from file");
        let mut conn = open_db(DatabaseLocation::Memory).expect("Failed to open database.");
        create_tables(&mut conn).expect("Failed to create tables???");
        insert_scrobbles(&mut conn, response.recent_tracks).expect("Failed to insert scrobbles.");
    }

    // Create last.fm api client
    // let mut client = LastFM::new(&opt.api_key, &opt.api_secret);
    // if let Ok(response) = client.recent_tracks(&opt.username) {
    //     // Serialize scrobbles
    //     if let Ok(file) = std::fs::File::create("response.json") {
    //         serde_json::to_writer(&file, &response).expect("Failed to write response");
    //     } else {
    //         eprintln!("Oh no! Failed to open file.");
    //     }
    // }
}
