use std::fs::File;
use std::io::BufReader;
use rand::Rng;
use rusqlite::{named_params, Row};
use serde::{Deserialize, Serialize};
use crate::database::*;


pub fn create_table(conn: &Conn) {
	create_table_creatures_types(conn);
	create_table_creatures_types_awards(conn);

	create_table_creatures(conn);
	create_table_creatures_awards(conn);
	create_table_creatures_damage(conn);
}

fn create_table_creatures(conn: &Conn) {
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
}

fn create_table_creatures_types_awards(conn: &Conn) {
	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_AWARDS_TABLE}(
			id INTEGER PRIMARY KEY,
			creature_type INTEGER,


			FOREIGN KEY (creature_type) REFERENCES {CREATURES_TYPES_TABLE}(type) ON DELETE CASCADE
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

fn create_table_creatures_awards(conn: &Conn) {
	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_AWARDS_TABLE}(
			id INTEGER PRIMARY KEY,
			damage INTEGER,
            account_id INTEGER,
            creature_id INTEGER,

			FOREIGN KEY (creature_id) REFERENCES {CREATURES_TABLE}(id) ON DELETE CASCADE
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

fn create_table_creatures_damage(conn: &Conn) {
	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_DAMAGE_TABLE}(
			id INTEGER PRIMARY KEY,
			damage INTEGER,
            account_id INTEGER,
            creature_id INTEGER,

			FOREIGN KEY (creature_id) REFERENCES {CREATURES_TABLE}(id) ON DELETE CASCADE
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}

fn create_table_creatures_types(conn: &Conn) {
	#[derive(Deserialize)]
	struct CreatureType {
		name: String,
		health: usize,
		health_per_level: usize,
	}

	let statement = format!("
		CREATE TABLE IF NOT EXISTS {CREATURES_TYPES_TABLE}(
			type INTEGER PRIMARY KEY,
			name TEXT,
			health INTEGER,
			health_per_level INTEGER
		)
	");
	conn.execute(statement.as_str(), ()).unwrap();

	let statement = format!("
		INSERT OR REPLACE INTO {CREATURES_TYPES_TABLE}(type, name, health, health_per_level) VALUES (:type, :name, :health, :health_per_level)
	");

	for (index, creature) in serde_json::from_reader::<_, Vec<CreatureType>>(BufReader::new(File::open("assets/creatures.json").unwrap())).unwrap().iter().enumerate() {
		conn.execute(&statement, named_params! {":type": index, ":name": creature.name, ":health": creature.health, ":health_per_level": creature.health_per_level}).unwrap();
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
	pub id: usize,
	pub name: String,
	pub level: usize,
	pub health: usize,
	pub damage: usize,
}
impl Creature {
	pub fn from_row(row: &Row) -> Self {
		Self {
			id: row.get(0).unwrap(),
			name: row.get(1).unwrap(),
			level: row.get(2).unwrap(),
			health: row.get(3).unwrap(),
			damage: row.get(4).unwrap(),
		}
	}
}

pub fn list(conn: &Conn) -> Vec<Creature> {
	let statement = format!("
		WITH damages AS (
			SELECT COALESCE(SUM(damage), 0) AS damage, {CREATURES_TABLE}.id
			FROM {CREATURES_TABLE}
			LEFT JOIN {CREATURES_DAMAGE_TABLE} ON {CREATURES_TABLE}.id = {CREATURES_DAMAGE_TABLE}.creature_id
			GROUP BY creatures.id
		)
		SELECT {CREATURES_TABLE}.id, name, level, {CREATURES_TABLE}.health, damages.damage
		FROM {CREATURES_TABLE}
		LEFT JOIN damages ON {CREATURES_TABLE}.id =  damages.id
		LEFT JOIN {CREATURES_TYPES_TABLE} ON {CREATURES_TABLE}.type = {CREATURES_TYPES_TABLE}.type
	");

	let mut statement = conn.prepare(statement.as_str()).unwrap();
	let rows = statement.query_map([], |row|{
		Ok(Creature::from_row(row))
	}).unwrap();

	rows.map(|row| row.unwrap()).collect()
}

pub fn attack(conn: &Conn, token: String, creature_id: usize) {
	fn try_kill(conn: &Conn, creature_id: usize) {
		let statement = format!("
			SELECT SUM(damage)
			FROM {CREATURES_DAMAGE_TABLE}
			WHERE creature_id = :creature_id
		");
		let mut statement = conn.prepare(&statement).unwrap();

		let rows = statement.query_map(named_params! {":creature_id": creature_id}, |row|{
			row.get::<_, usize>(0)
		}).unwrap();
		let mut damage: usize = rows.last().unwrap().unwrap();


		let statement = format!("
			SELECT health
			FROM {CREATURES_TABLE}
			WHERE id = :creature_id
		");
		let mut statement = conn.prepare(&statement).unwrap();
		let rows = statement.query_map(named_params! {":creature_id": creature_id}, |row|{
			row.get::<_, usize>(0)
		}).unwrap();

		let health: usize = rows.last().unwrap().unwrap();

		if damage >= health {
			#[derive(Debug)]
			struct Impact {
				account_id: usize,
				impact: f64,
			}

			impl Impact {
				fn from_row(row: &Row) -> Result<Self, rusqlite::Error> {
					Ok(Self {
						account_id: row.get(0)?,
						impact: row.get(1)?
					})
				}
			}

			let statement = format!("
				WITH damage_overall AS (
					SELECT SUM(damage) AS damage_overall
					FROM {CREATURES_DAMAGE_TABLE}
					WHERE creature_id = :creature_id
				)

				SELECT account_id, SUM(damage) / (SELECT * FROM damage_overall)
				FROM {CREATURES_DAMAGE_TABLE}
				WHERE creature_id = :creature_id
				GROUP BY account_id
			");
			let mut statement = conn.prepare(&statement).unwrap();

			let rows = statement.query_map(named_params! {":creature_id": creature_id}, |row|{
				Impact::from_row(row)
			}).unwrap();
			let impact: Vec<_> = rows.map(|x|x.unwrap()).collect();

			println!("\n\n\nKILLED CREATURE{:?}\n\n\n", impact);

			let statement = format!("
				UPDATE {ACCOUNTS_TABLE} SET balance = balance + :impact WHERE id = :account_id
			");

			for impact in impact {
				println!("{}", impact.impact);
				conn.execute(&statement, named_params! {":impact": impact.impact * 10.0, ":account_id": impact.account_id}).unwrap();
			}

			let statement = format!("
				DELETE FROM {CREATURES_TABLE}
				WHERE id = :creature_id
			");
			conn.execute(&statement, named_params! {":creature_id": creature_id}).unwrap();
		}
	}


	if let Some(account_id) = crate::database::sessions::get_account_id(conn, &token) {
		let damage = 5;

		let statement = format!("
			INSERT INTO {CREATURES_DAMAGE_TABLE}(damage, account_id, creature_id) VALUES (:damage, :account_id, :creature_id)
		");
		conn.execute(&statement, named_params! {":damage": damage, ":creature_id": creature_id, ":account_id": account_id}).unwrap();

		try_kill(conn, creature_id);
	}
}

pub fn spawn(conn: &Conn) {
	let statement = format!("
		WITH creatures AS (
			SELECT * FROM {CREATURES_TYPES_TABLE} WHERE {CREATURES_TYPES_TABLE}.type = :type
		)
		INSERT INTO {CREATURES_TABLE}(type, level, health) VALUES (:type, :level, (SELECT health FROM creatures) + (SELECT health_per_level FROM creatures) * :level)
	");

	let type_value = rand::thread_rng().gen_range(0..3);
	let level = rand::thread_rng().gen_range(0..15);


	let mut statement = conn.prepare(statement.as_str()).unwrap();
	statement.execute(named_params! {":type": type_value, ":level": level}).unwrap();
}