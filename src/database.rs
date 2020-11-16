use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv;
use super::error::UserFacingError;

pub fn establish_connection() -> Result<SqliteConnection, UserFacingError> {
    let db_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in env");
    
    return SqliteConnection::establish(&db_url)
        .map_err(|_e| UserFacingError::InternalError)
}