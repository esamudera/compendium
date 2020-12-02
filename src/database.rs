use dotenv;
use super::error::UserFacingError;

use rusqlite::Connection;

pub fn get_connection() -> Result<Connection, UserFacingError> {
    let db_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in env");
    
    return Connection::open(db_url)
        .map_err(|_e| UserFacingError::InternalError)
}