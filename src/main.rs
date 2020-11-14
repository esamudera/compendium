use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod route;
mod playbook;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(route::config_route)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}