use rusqlite::{params, Rows};
use chrono::{Utc};

use std::error::Error;

use crate::playbook::model::Playbook;
use crate::database;

pub fn insert_new_playbook(title: &str) -> Result<i64, Box<dyn Error>> {
    let connection = database::get_connection()?;

    connection
        .execute(
            "INSERT INTO playbook (title, create_time, update_time) VALUES (?1, ?2, ?3)",
            params![
                title,
                Utc::now().timestamp(),
                Utc::now().timestamp()
            ]
        )?;

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

    let playbooks = parse_playbook_row(&mut rows)?;

    Ok(playbooks)
}

pub fn delete_playbook(id: i64) -> Result<(), Box<dyn Error>> {
    let connection = database::get_connection()?;
    connection
        .execute(
            "DELETE FROM playbook WHERE id = ?1",
            params![id]
        )?;
    Ok(())
}

pub fn read_playbook(id: i64) -> Result<Playbook, Box<dyn Error>> {
    let connection = database::get_connection()?;

    let sql_query = 
        "SELECT
            id, title, create_time, update_time
        FROM playbook
        WHERE id = ?
        LIMIT 1";

    let mut statement = connection.prepare(&sql_query)?;
    let mut rows = statement.query(params![ id ])?;

    let playbooks = parse_playbook_row(&mut rows)?;
    let pb = playbooks.first().expect("Not found");
    
    Ok(pb.clone())
}

fn parse_playbook_row(rows: &mut Rows) -> Result<Vec<Playbook>, Box<dyn Error>> {
    let mut playbooks = Vec::new();

    while let Some(row) = rows.next()? {
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