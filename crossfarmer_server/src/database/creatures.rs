use std::fmt::format;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rand::Rng;
use rusqlite::{named_params, Row};
use crate::api::gameplay::creatures::messages::Creature;
use crate::database::*;


pub fn create_table(conn: &PooledConnection<SqliteConnectionManager>) {
	let statement = format!("
		CREATE TABLE IF NOT EXISTS {CREATURES_TYPES_TABLE}(
			type INTEGER PRIMARY KEY,
			name TEXT
		)
	");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");


	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_TABLE}(
            id INTEGER PRIMARY KEY,
			type INTEGER,
			level INTEGER,
			health INTEGER NOT NULL,

			FOREIGN KEY (type) REFERENCES {CREATURES_TYPES_TABLE}(type)
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");


	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_DAMAGE_TABLE}(
			id INTEGER PRIMARY KEY,
			damage INTEGER,
            account_id INTEGER,
            creature_id INTEGER,

			FOREIGN KEY (creature_id) REFERENCES {CREATURES_TABLE}(id)
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

impl Creature {
	pub fn from_row(row: &Row) -> Self {
		Self {
			id: row.get(0).unwrap(),
			name: row.get(1).unwrap(),
			level: row.get(2).unwrap(),
			health: row.get(3).unwrap(),
		}
	}
}

pub fn list(pool: &Pool) -> Vec<Creature> {
	let pool = pool.clone();
	let conn = pool.get().unwrap();

	let statement = format!("
        SELECT id, name, level, health FROM {CREATURES_TABLE}
		LEFT JOIN {CREATURES_TYPES_TABLE} ON {CREATURES_TABLE}.type = {CREATURES_TYPES_TABLE}.type
    ");

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	let rows = statement.query_map([], |row|{
		Ok(Creature::from_row(row))
	}).unwrap();

	rows.map(|row| row.unwrap()).collect()
}

pub fn attack(id: usize, pool: &Pool) {
	let pool = pool.clone();
	let conn = pool.get().unwrap();

	let statement = format!("
		SELECT health FROM {CREATURES_TABLE}
		WHERE id == :id
	");

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	let rows = statement.query_map(named_params! {":id": id}, |row| row.get(0)).unwrap();

	let damage = 5;

	let mut health: usize = rows.last().unwrap().unwrap();
	if health <= damage {
		health = 0;
	} else {
		health -= damage;
	}

	let statement = format!("
		UPDATE {CREATURES_TABLE}
		SET health = :health
		WHERE id == :id
	");

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	statement.execute(named_params! {":id": id, ":health": health}).unwrap();
}

pub fn spawn(pool: &Pool) {
	let pool = pool.clone();
	let conn = pool.get().unwrap();

	let statement = format!("
		INSERT INTO {CREATURES_TABLE}(type, level, health) VALUES (:type, :level, :health)
	");

	let type_value = rand::thread_rng().gen_range(1..=3);
	let level = rand::thread_rng().gen_range(1..=100);
	let health = rand::thread_rng().gen_range(1..=1000);

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	statement.execute(named_params! {":type": type_value, ":level": level, ":health": health}).unwrap();
}