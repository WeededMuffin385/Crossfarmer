use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::named_params;
use uuid::Uuid;
use crate::database::{Pool, SESSIONS_TABLE};

pub fn session_create(pool: &Pool, mail: &str) -> Uuid {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let token = Uuid::new_v4();
    let date = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let statement = format!("
        INSERT INTO {SESSIONS_TABLE} (token, mail, date) VALUES (:token, :mail, :date)
    ");


    conn.execute(statement.as_str(), named_params! {":token": token.to_string(), ":mail": mail, ":date": date}).unwrap();
    token
}

pub fn session_exists(pool: &Pool, mail: &str) -> bool {
    return true;
}