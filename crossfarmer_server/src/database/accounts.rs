use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::named_params;
use crate::database::*;

pub fn create_table(conn: &Conn) {
    let statement = format!("
        CREATE TABLE IF NOT EXISTS {ACCOUNTS_TABLE}(
            id INTEGER PRIMARY KEY,
            mail TEXT UNIQUE NOT NULL,
            username TEXT,
            password TEXT
        )
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

pub fn create(conn: &Conn, mail: &str, username: &str, password: &str){
    println!("[Account added] Mail: {mail}| username: {username}| password: {password}");

    let statement = format!("
        INSERT INTO {ACCOUNTS_TABLE} (mail, username, password) VALUES (:mail, :username, :password)
    ");

    conn.execute(
        statement.as_str(),
        named_params! {":mail": mail, ":username": username, ":password": password}
    ).unwrap();
}

pub fn get_id(conn: &Conn, mail: &str) -> usize {
    let statement = format!("
        SELECT id FROM {ACCOUNTS_TABLE} WHERE mail = :mail
    ");

    let mut statement = conn.prepare(&statement).unwrap();
    let rows = statement.query_map(named_params! {":mail": mail}, |row| row.get(0)).unwrap();
    rows.last().unwrap().unwrap()
}

pub fn authorize(conn: &Conn, mail: &str, password: &str) -> bool {
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

pub fn exists(conn: &Conn, mail: &str) -> bool {
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