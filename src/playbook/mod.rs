use actix_web::{HttpRequest, HttpResponse, error, Result};
use super::database;

pub async fn handle_new_playbook(req: HttpRequest) -> Result<&'static str> {
    database::get_connection()?;
    Ok("coba")
}