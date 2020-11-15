use actix_web::{HttpRequest, HttpResponse, error, Result};

pub async fn handle_new_playbook(req: HttpRequest) -> Result<&'static str> {
    Ok("coba")
}