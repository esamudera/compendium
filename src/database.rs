#[macro_use]

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv;
use super::error::UserFacingError;

pub fn get_connection() -> Result<SqliteConnection, UserFacingError> {
    let db_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in env");
    
    SqliteConnection::establish(&db_url)
        .map_err(|_e| UserFacingError::InternalError)
}