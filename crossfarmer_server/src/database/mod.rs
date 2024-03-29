use r2d2_sqlite::SqliteConnectionManager;

pub mod accounts;
pub mod sessions;
pub mod guilds;

pub type Pool = r2d2::Pool<SqliteConnectionManager>;

pub const ACCOUNTS_TABLE: &str = "accounts";
pub const SESSIONS_TABLE: &str = "sessions";
pub const CURRENCY_TABLE: &str = "currency";
pub const AUCTION_TABLE: &str = "auction";
pub const GUILDS_TABLE: &str = "guilds";
pub const GUILDS_MEMBERS_TABLE: &str = "guilds_members";

pub fn create() -> Pool {
    let manager = SqliteConnectionManager::file("database.db");
    let pool = r2d2::Pool::new(manager).expect("Unable to create database pool!");
    create_tables(&pool);
    return pool;
}

fn create_tables(pool: &Pool) {
    let conn = pool.get().unwrap();
    accounts::create_table(&conn);
    guilds::create_table(&conn);
    sessions::create_table(&conn);
}