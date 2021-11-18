use clap::Parser;
use rustfm::{user::recent_tracks::Track, Client};

mod api;
mod fetch;

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
    secret_key: String,
}

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Parse program options
    let opt: Opts = Opts::parse();

    // Create last.fm api client
    let mut client = Client::new(&opt.api_key);

    if let Ok(scrobbles) = fetch::fetch_scrobbles(&mut client, &opt.username) {
        // Serialize scrobbles
        if let Ok(file) = std::fs::File::create("scrobbles.json") {
            serde_json::to_writer(&file, &scrobbles);
        } else {
            eprintln!("Oh no! Failed to open file.");
        }
    }
}
