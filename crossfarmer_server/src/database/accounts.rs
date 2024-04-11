use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use crate::database::*;

pub fn create_table(conn: &PooledConnection<SqliteConnectionManager>) {
    let statement = format!("
        CREATE TABLE IF NOT EXISTS {ACCOUNTS_TABLE}(
            id TEXT PRIMARY KEY,
            mail TEXT UNIQUE NOT NULL,
            username TEXT,
            password TEXT
        )
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

pub fn create(pool: &Pool, mail: &str, username: &str, password: &str){
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    println!("[Account added] Mail: {}| username: {}| password: {}", mail, username, password);

    let statement = format!("
        INSERT INTO {ACCOUNTS_TABLE} (mail, username, password) VALUES (:mail, :username, :password)
    ");

    conn.execute(
        statement.as_str(),
        named_params! {":mail": mail, ":username": username, ":password": password}
    ).unwrap();
}

pub fn authorize(pool: &Pool, mail: &str, password: &str) -> bool {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let statement = format!("
        SELECT COUNT(*) FROM {ACCOUNTS_TABLE} WHERE mail = :mail AND password = :password
    ");

    let mut statement = conn.prepare(statement.as_str()).expect("wrong request");
    let result = statement.query_map(
        named_params! {":mail": mail, ":password": password},
        |row| row.get(0)
    ).unwrap();

    let count: i32 = result.last().unwrap().unwrap();
    count == 1
}

pub fn exists(pool: &Pool, mail: &str) -> bool {
    let pool = pool.clone();
    let conn = pool.get().unwrap();

    let statement = format!("
        SELECT COUNT(*) FROM {ACCOUNTS_TABLE} WHERE mail = :mail
    ");

    let mut statement = conn.prepare(statement.as_str()).expect("wrong request");
    let result = statement.query_map(
        named_params! {":mail": mail},
        |row| row.get(0)
    ).unwrap();

    let count: i32 = result.last().unwrap().unwrap();
    count == 1
}