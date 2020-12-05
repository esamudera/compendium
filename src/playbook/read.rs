use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::playbook::dao;
use crate::error::UserFacingError;

#[derive(Deserialize)]
pub struct ReadPlaybookRequest {
    id: i64
}

pub async fn handle(query: web::Path<ReadPlaybookRequest>) -> Result<HttpResponse> {
    let playbook = dao::read_playbook(query.id)
        .map_err(|_e| UserFacingError::InternalError)?;
    Ok(
        HttpResponse::Ok()
            .json(playbook)
    )
}