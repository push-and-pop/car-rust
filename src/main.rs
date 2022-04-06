use actix_web::{get, web, App, HttpServer, Responder};
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
mod server;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
