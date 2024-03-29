use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::database::*;

struct Guild {
    id: usize,
    name: String,
}

struct Member {
    mail: String,
    guild_id: usize,
    guild_role: usize,
}

pub fn create_table(conn: &PooledConnection<SqliteConnectionManager>) {
    let statement = format!("
        CREATE TABLE IF NOT EXISTS {GUILDS_TABLE}(
            id INTEGER PRIMARY KEY,
            guild_name TEXT
        )
    ");

    conn.execute(statement.as_str(), ()).expect("unable to create guilds table");

    let statement = format!("
        CREATE TABLE IF NOT EXISTS {GUILDS_MEMBERS_TABLE}(
            mail TEXT PRIMARY KEY,
            guild_id INTEGER,
            user_type INTEGER
        )
    ");

    conn.execute(statement.as_str(), ()).expect("unable to create guilds table");
}