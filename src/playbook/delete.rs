use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

use crate::error::UserFacingError;

use crate::playbook::dao;

#[derive(Deserialize)]
pub struct DeletePlaybookRequest {
    id: i64
}

pub async fn handle(
    req: web::Path<DeletePlaybookRequest>
) -> Result<HttpResponse> {
    dao::delete_playbook(req.id)
        .map_err(|_e| UserFacingError::InternalError)?;
    Ok(
        HttpResponse::NoContent().finish()
    )
}