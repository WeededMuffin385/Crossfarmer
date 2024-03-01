pub mod accounts;
pub mod sessions;

use r2d2_sqlite::SqliteConnectionManager;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub const ACCOUNTS_TABLE: &str = "accounts";
pub const SESSIONS_TABLE: &str = "sessions";

pub fn create_database_pool() -> Pool {
    let manager = SqliteConnectionManager::file("database.db");
    let pool = r2d2::Pool::new(manager).expect("Unable to create database pool!");

    configure_database(&pool);
    return pool;
}

pub fn configure_database(pool: &Pool)  {
    let pool = pool.clone();
    let conn = pool.get().expect("something went wrong");

    let statement = format!("
        CREATE TABLE IF NOT EXISTS {ACCOUNTS_TABLE}(
            mail TEXT PRIMARY KEY,
            username TEXT,
            password TEXT
        )
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");

    let statement = format!("
        CREATE TABLE IF NOT EXISTS {SESSIONS_TABLE}(
            token TEXT PRIMARY KEY,
            mail TEXT,
            date DATE,

            FOREIGN KEY (mail) REFERENCES {ACCOUNTS_TABLE}(mail)
        )
    ");
    conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

