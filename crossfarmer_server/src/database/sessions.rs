use std::time::{SystemTime, UNIX_EPOCH};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use uuid::Uuid;

use crate::database::*;

struct Session {
    token: Uuid,
    mail: String,
    date: u64,
}

pub fn create_table(conn: &PooledConnection<SqliteConnectionManager>) {
    let statement = format!("
        CREATE TABLE IF NOT EXISTS {SESSIONS_TABLE}(
            token TEXT PRIMARY KEY,
            mail TEXT,
            date DATE,

            FOREIGN KEY (mail) REFERENCES {ACCOUNTS_TABLE}(mail)
        );
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");

    let statement = format!("
        DELETE FROM {SESSIONS_TABLE}
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

pub fn create(pool: &Pool, mail: &str) -> Uuid {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let token = Uuid::new_v4();
    let date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let statement = format!("
        INSERT INTO {SESSIONS_TABLE} (token, mail, date) VALUES (:token, :mail, :date)
    ");


    conn.execute(statement.as_str(), named_params!{":token": token.to_string(), ":mail": mail, ":date": date}).unwrap();
    token
}

pub fn exists(pool: &Pool, mail: &str) -> bool {
    return true;
}