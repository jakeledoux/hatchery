use super::api::*;
use rusqlite::{params, Connection};

pub enum DatabaseLocation {
    Memory,
    Disk(String),
}

pub fn open_db(location: DatabaseLocation) -> rusqlite::Result<Connection> {
    match location {
        DatabaseLocation::Memory => Connection::open_in_memory(),
        DatabaseLocation::Disk(path) => Connection::open(path),
    }
}

pub fn close_db(conn: Connection) -> rusqlite::Result<(), (Connection, rusqlite::Error)> {
    conn.close()
}

// id integer primary key,
// name text not null,
// mbid text,
// artist text not null,
// artist_mbid text,
// album text not null,
// album_mbid text,
// timestamp datetime

pub fn create_tables(conn: &mut Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS scrobbles", [])?;
    conn.execute(
        "CREATE TABLE scrobbles (
            id             INTEGER PRIMARY KEY,
            name           TEXT NOT NULL,
            mbid           TEXT,
            artist         TEXT NOT NULL,
            artist_mbid    TEXT,
            album          TEXT NOT NULL,
            album_mbid     TEXT,
            timestamp      DATETIME
        )",
        [],
    )?;
    conn.execute("DROP TABLE IF EXISTS friends", [])?;
    conn.execute(
        "CREATE TABLE friends (
            id             INTEGER PRIMARY KEY,
            name           TEXT NOT NULL,
            real_name      TEXT,
            country        TEXT NOT NULL,
            subscriber     BOOLEAN,
            registered     DATETIME
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_scrobbles(
    conn: &mut Connection,
    scrobbles: Vec<Track>,
) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;

    {
        let mut statement = trans.prepare(
            "INSERT INTO scrobbles
                (name, mbid, artist, artist_mbid, album, album_mbid, timestamp)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
        )?;

        for track in scrobbles {
            // Skip in-progress/unfinished scrobble
            if let Some(attributes) = track.attributes {
                if attributes.now_playing {
                    continue;
                }
            }

            statement.execute(params![
                track.name,
                track.mbid,
                track.artist.name,
                track.artist.mbid,
                track.album.name,
                track.album.mbid,
                match track.date {
                    Some(date) => Some(date.datetime),
                    None => None,
                }
            ])?;
        }
    }
    trans.commit()
}

pub fn insert_friends(conn: &mut Connection, friends: Vec<Friend>) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;

    {
        let mut statement = trans.prepare(
            "INSERT INTO friends
                (name, real_name, country, subscriber, registered)
                VALUES (?1, ?2, ?3, ?4, ?5)
            ",
        )?;

        for friend in friends {
            statement.execute(params![
                friend.name,
                friend.real_name,
                friend.country,
                friend.subscriber,
                friend.registered.datetime
            ])?;
        }
    }
    trans.commit()
}
