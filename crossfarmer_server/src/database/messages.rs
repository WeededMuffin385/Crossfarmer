use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, Row};
use crate::api::messages::messages::Message;

use crate::database::{ACCOUNTS_TABLE, Conn, MESSAGES_TABLE, Pool};

pub fn create_table(conn: &Conn) {
	let statement = format!("
        CREATE TABLE IF NOT EXISTS {MESSAGES_TABLE}(
            id TEXT PRIMARY KEY,
            account_id INTEGER NOT NULL,
            message TEXT
        )
    ");
	conn.execute(&statement, []).expect("unable to configure via prepare");
}

impl Message {
	pub fn from_row(row: &Row) -> Self {
		Self {
			username: row.get(0).unwrap(),
			message: row.get(1).unwrap(),
		}
	}
}

pub fn list(pool: &Pool) -> Vec<Message> {
	let pool = pool.clone();
	let conn = pool.get().unwrap();

	let statement = format!("
        SELECT username, message FROM {MESSAGES_TABLE}
		LEFT JOIN {ACCOUNTS_TABLE} ON {MESSAGES_TABLE}.account_id = {ACCOUNTS_TABLE}.id
    ");

	let mut statement = conn.prepare(&statement).unwrap();
	let rows = statement.query_map([], |row|{
		Ok(Message::from_row(row))
	}).unwrap();

	rows.map(|row| row.unwrap()).collect()
}

pub fn send(pool: &Pool, account_id: usize, message: String) {
	let pool = pool.clone();
	let conn = pool.get().unwrap();

	let statement = format!("
		INSERT INTO {MESSAGES_TABLE}(account_id, message) VALUES (:account_id, :message)
	");

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	statement.execute(named_params! {":account_id": account_id, ":message": message}).unwrap();
}