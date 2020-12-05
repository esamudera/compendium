use dotenv;

use rusqlite::Connection;
use std::error::Error;

pub fn get_connection() -> Result<Connection, Box<dyn Error>> {
    let db_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in env");
    let conn = Connection::open(db_url)?;
    Ok(conn)
}