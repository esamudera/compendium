use actix_web::{web, http, HttpResponse, Result};
use chrono::{Utc};
use serde::Deserialize;

use crate::database;
use crate::error::UserFacingError;
use crate::shared::input;

use rusqlite::{params};

#[derive(Deserialize)]
pub struct NewPlaybookRequest {
    title: String
}

pub async fn handle(
    req: web::Json<NewPlaybookRequest>
) -> Result<HttpResponse> {

    let connection = database::get_connection()?;

    connection
        .execute(
            "INSERT INTO playbook (title, create_time, update_time) VALUES (?1, ?2, ?3)",
            params![
                input::clean(&req.title),
                Utc::now().timestamp(),
                Utc::now().timestamp()
            ]
        )
        .map_err(|_e| UserFacingError::InternalError)?;

    let last_id = connection.last_insert_rowid();
    let new_playbook_url = format!("/api/playbook/{}", last_id);

    Ok(
        HttpResponse::Created()
            .header(http::header::LOCATION, new_playbook_url)
            .finish()
    )
}