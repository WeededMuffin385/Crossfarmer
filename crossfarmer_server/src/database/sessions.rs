use std::time::{SystemTime, UNIX_EPOCH};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use uuid::Uuid;

use crate::database::*;

pub fn create_table(conn: &Conn) {
    let statement = format!("
        CREATE TABLE IF NOT EXISTS {SESSIONS_TABLE}(
            token TEXT PRIMARY KEY,
            account_id INTEGER,
            date DATE,

            FOREIGN KEY (account_id) REFERENCES {ACCOUNTS_TABLE}(id)
        );
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");

    let statement = format!("
        DELETE FROM {SESSIONS_TABLE}
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

pub fn create(conn: &Conn, mail: &str) -> Uuid {
    let token = Uuid::new_v4();
    let date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let statement = format!("
        INSERT INTO {SESSIONS_TABLE} (token, account_id, date) VALUES (:token, :account_id, :date)
    ");

    let account_id = crate::database::accounts::get_id(&conn, mail);

    conn.execute(statement.as_str(), named_params!{":token": token.to_string(), ":account_id": account_id, ":date": date}).unwrap();
    token
}

pub fn get_account_id(conn: &Conn, token: &str) -> Option<usize> {
    let statement = format!("
        SELECT account_id FROM {SESSIONS_TABLE} WHERE token = :token
    ");

    let mut statement = conn.prepare(statement.as_str()).unwrap();
    let rows = statement.query_map(named_params! {":token": token}, |row|row.get(0)).unwrap();

    if let Some(token) = rows.last() {
        token.unwrap()
    } else {
        None
    }
}