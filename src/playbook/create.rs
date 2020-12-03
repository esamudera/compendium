use actix_web::{web, http, HttpResponse, Result};
use chrono::{Utc};
use serde::Deserialize;

use crate::database;
use crate::shared::input;

use crate::playbook::dao;

#[derive(Deserialize)]
pub struct NewPlaybookRequest {
    title: String
}

pub async fn handle(
    req: web::Json<NewPlaybookRequest>
) -> Result<HttpResponse> {

    let new_title = input::clean(&req.title);
    let new_id = dao::insert_new_playbook(&new_title)?;
    
    let new_playbook_url = format!("/api/playbook/{}", new_id);

    Ok(
        HttpResponse::Created()
            .header(http::header::LOCATION, new_playbook_url)
            .finish()
    )
}