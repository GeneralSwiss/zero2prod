use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    email_client: EmailClient,
) -> std::io::Result<Server> {
    let web_data = web::Data::new(connection_pool);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health", web::get().to(health_check))
            .app_data(web_data.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

impl Application {
    pub async fn build(configuration: &Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database).await;

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid Sender email address");
        let email_client = EmailClient::new(
            aws_sdk_sesv2::Client::new(&aws_config::load_from_env().await),
            sender_email,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    
    configure_database(configuration).await
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database...");

    // Migrate Database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to database...");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to run migrations...");

    connection_pool
}

pub struct Application {
    port: u16,
    server: Server,
}
