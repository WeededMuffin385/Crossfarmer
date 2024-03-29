use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::database::*;

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
            account_id INTEGER PRIMARY KEY,
            member_type INTEGER
            guild_id INTEGER
        )
    ");
    conn.execute(statement.as_str(), ()).expect("unable to create guilds table");
}