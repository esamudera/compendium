use actix_web::{web, http, HttpRequest, HttpResponse, Result};

pub async fn handle(req: HttpRequest) -> Result<HttpResponse> {
    Ok(
        HttpResponse::Ok()
            .finish()
    )
}