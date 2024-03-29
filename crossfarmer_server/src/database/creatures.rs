use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use crate::database::*;


pub fn create_table(conn: &PooledConnection<SqliteConnectionManager>) {
	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_TABLE}(
            id INTEGER PRIMARY KEY,
            health_max INTEGER,
			health_current INTEGER
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");


	let statement = format!("
        CREATE TABLE IF NOT EXISTS {CREATURES_DAMAGE_TABLE}(
			damage_dealt INTEGER,
            creature_id INTEGER,
            account_id INTEGER,

			PRIMARY KEY(creature_id, account_id)
        )
    ");
	conn.execute(statement.as_str(), ()).expect("unable to configure via prepare");
}