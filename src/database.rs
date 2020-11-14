use rusqlite::{ Connection, Error, OpenFlags };

use super::error;

pub fn get_connection() -> Result<Connection, error::UserFacingError> {
    Connection::open_with_flags("coba.db", OpenFlags::SQLITE_OPEN_READ_WRITE)
        .map_err(|_e| error::UserFacingError::InternalError)
}