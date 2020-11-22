use actix_web::{web, http, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    sort_by: Option<String>,
    order: Option<String>,
    limit: Option<i32>
}

pub async fn handle(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let sort_by = query.sort_by.as_deref().unwrap_or("create_time");
    let order = query.order.as_deref().unwrap_or("desc");
    let limit : i32 = query.limit.unwrap_or(10);

    Ok(
        HttpResponse::Ok()
            .body(format!("BANG! {} {} {}", sort_by, order, limit))
    )
}