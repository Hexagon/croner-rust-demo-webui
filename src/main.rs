// src/main.rs
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // Define the application's routes
    let app = Router::new()
        // API route to test cron expressions
        .route("/api/test", get(handlers::test_cron_handler))
        // Static file serving for the frontend.
        // It will serve files from the `static` directory.
        // The `nest_service` call will handle requests to `/` by serving `index.html`.
        .nest_service("/", ServeDir::new("static"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(">> Croner API Demo listening on http://{}", addr);
    println!(">> Visit http://127.0.0.1:3000 to use the Cron Tester.");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}