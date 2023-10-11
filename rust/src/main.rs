use crate::handlers::{get::get_handler, post::post_handler};
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod handlers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenvy::var("DATABASE_URL")?)
        .await?;
    let routes = Router::new()
        .route("/tb01", get(get_handler).post(post_handler))
        .with_state(pool.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor iniciado em: {addr}");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;

    Ok(())
}
