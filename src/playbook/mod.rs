use actix_web::{HttpRequest, HttpResponse};

pub async fn handle_new_playbook(req: HttpRequest) -> HttpResponse {
    HttpResponse::NoContent().finish()
}