use super::api::*;
use rusqlite::Connection;
use sea_query::{ColumnDef, Iden, Query, SqliteQueryBuilder, Table};

sea_query::sea_query_driver_rusqlite!();
use sea_query_driver_rusqlite::RusqliteValues;
use std::error::Error;

#[derive(Iden)]
pub enum Scrobble {
    Table,
    Id,
    Name,
    MBID,
    Artist,
    ArtistMBID,
    Album,
    AlbumMBID,
    Timestamp,
}

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

// id integer primary key,
// name text not null,
// mbid text,
// artist text not null,
// artist_mbid text,
// album text not null,
// album_mbid text,
// timestamp datetime

pub fn create_tables(conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    let sql = [
        Table::drop()
            .table(Scrobble::Table)
            .if_exists()
            .build(SqliteQueryBuilder),
        Table::create()
            .table(Scrobble::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Scrobble::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Scrobble::Name).string().not_null())
            .col(ColumnDef::new(Scrobble::MBID).string())
            .col(ColumnDef::new(Scrobble::Artist).string().not_null())
            .col(ColumnDef::new(Scrobble::ArtistMBID).string())
            .col(ColumnDef::new(Scrobble::Album).string().not_null())
            .col(ColumnDef::new(Scrobble::AlbumMBID).string())
            .col(ColumnDef::new(Scrobble::Timestamp).date_time())
            .build(SqliteQueryBuilder),
    ]
    .join(";");
    Ok(conn.execute_batch(&sql)?)
}

pub fn insert_scrobbles(
    conn: &mut Connection,
    scrobbles: RecentTracks,
) -> Result<(), Box<dyn Error>> {
    todo!()
}
