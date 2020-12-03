use rusqlite::{Connection, params};
use chrono::{Utc};

use std::error::Error;

use crate::playbook::model::Playbook;
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

pub fn get_playbook_list(limit: i32, sort: &str, sort_order: &str) -> Result<Vec<Playbook>, Box<dyn Error>> {
    let connection = database::get_connection()?;

    let sql_query = format!(
        "SELECT
            id, title, create_time, update_time
        FROM playbook
        ORDER BY ? {}
        LIMIT ?",
        sort_order
    );

    let mut statement = connection.prepare(&sql_query)?;
    let mut rows = statement.query(params![ sort, limit ])?;

    let mut playbooks = Vec::new();

    while let Some(row) = rows.next().map_err(|_e| UserFacingError::InternalError)? {
        let playbook = Playbook {
            id: row.get(0)?,
            title: row.get(1)?,
            create_time: row.get(2)?,
            update_time: row.get(3)?
        };
        playbooks.push(playbook)
    }

    Ok(playbooks)
}