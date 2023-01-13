use crate::domain::Subscriber;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(subscriber, connection_pool),
    fields(
        subscriber_email = %subscriber.email.as_ref(),
        subscriber_name = %subscriber.name.as_ref(),
    )
)]
pub async fn subscribe(
    subscriber: web::Form<Subscriber>,
    connection_pool: web::Data<PgPool>,
) -> impl Responder {
    match insert_subscriber(connection_pool.get_ref(), &subscriber).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(subscriber, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, subscriber: &Subscriber) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at, status)
        VALUES ($1, $2, $3, $4, 'confirmed')
        "#,
        Uuid::new_v4(),
        subscriber.email.as_ref(),
        subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
