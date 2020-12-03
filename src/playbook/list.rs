use actix_web::{web, http, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use std::fmt::Debug;
use crate::playbook::dao;
use crate::error::UserFacingError;

#[derive(Deserialize)]
pub struct QueryParams {
    sort_by: Option<String>,
    order: Option<String>,
    limit: Option<i32>
}

pub async fn handle(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let input_limit : i32 = query.limit.unwrap_or(10);

    let sort_by = match query.sort_by.as_deref() {
        Some("id") => "id",
        Some("create_time") => "create_time",
        Some("update_time") => "update_time",
        _ => "id"
    };

    let order = match query.order.as_deref() {
        Some("desc") => "desc",
        Some("asc") => "asc",
        _ => "desc"
    };

    let playbook_list = dao::get_playbook_list(input_limit, &sort_by, &order)
        .map_err(|_e| UserFacingError::InternalError)?;

    Ok(
        HttpResponse::Ok()
            .json(playbook_list)
    )
}