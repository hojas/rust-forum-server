use std::net::SocketAddr;
use axum::{Server};
use dotenvy::dotenv;

mod my_tracing;
mod pool;
mod migrations;
mod session;
mod router;

pub mod schema;
pub mod models;
pub mod controllers;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    // set up tracing
    my_tracing::tracing();

    // set up connection pool
    let pool = pool::get_pool();

    // run the migrations on server startup
    migrations::migrate(pool.clone()).await;

    // get session layer
    let session_layer = session::session_layer();

    // build application with routes
    let app = router::routes(pool).layer(session_layer);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    println!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
