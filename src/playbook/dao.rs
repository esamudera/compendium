use rusqlite::{Connection, params};
use chrono::{Utc};

use crate::error::UserFacingError;
use crate::database;

pub fn insert_new_playbook(title: &str) -> Result<i64, UserFacingError> {
    let connection = database::get_connection()?;

    connection
        .execute(
            "INSERT INTO playbook (title, create_time, update_time) VALUES (?1, ?2, ?3)",
            params![
                title,
                Utc::now().timestamp(),
                Utc::now().timestamp()
            ]
        )
        .map_err(|_e| UserFacingError::InternalError)?;

    let new_id = connection.last_insert_rowid();
    return Ok(new_id);
}