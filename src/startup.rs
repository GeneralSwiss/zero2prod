use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, connection_pool: PgPool) -> std::io::Result<Server> {
    let web_data = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health", web::get().to(health_check))
            .app_data(web_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
