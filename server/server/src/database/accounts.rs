use rusqlite::named_params;
use crate::database::{ACCOUNTS_TABLE, Pool};
pub fn account_exists(pool: &Pool, mail: &str) -> bool {
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

pub fn account_create(pool: &Pool, mail: &str, username: &str, password: &str) {
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

pub fn account_password(pool: &Pool, mail: &str, password: &str) -> bool {
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