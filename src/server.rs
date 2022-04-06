use actix_web::{get, web, App, HttpServer, Responder};
pub async fn run_server() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 9090))?
        .run()
        .await
}
