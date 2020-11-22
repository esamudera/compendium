use actix_web::{web, http, HttpRequest, HttpResponse, Result};
use serde::Deserialize;

use std::fmt::Debug;

#[derive(Deserialize)]
pub struct QueryParams {
    sort_by: Option<String>,
    order: Option<String>,
    limit: Option<i32>
}

#[derive(Debug)]
enum SortOrder {
    Ascending,
    Descending
}

pub async fn handle(query: web::Query<QueryParams>) -> Result<HttpResponse> {
    let input_sort_by = query.sort_by.as_deref().unwrap_or("create_time");
    let input_order = query.order.as_deref().unwrap_or("desc");
    let input_limit : i32 = query.limit.unwrap_or(10);

    let order = enumify_order(input_order);

    Ok(
        HttpResponse::Ok()
            .body(format!("BANG! {} {:?} {}", input_sort_by, order, input_limit))
    )
}

fn enumify_order(input_order: &str) -> SortOrder {
    return match input_order {
        "asc" => SortOrder::Ascending,
        "desc" => SortOrder::Descending,
        _ => SortOrder::Descending
    };
}