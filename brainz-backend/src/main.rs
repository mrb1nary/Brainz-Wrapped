use axum::Router;
use tower_http::cors::{CorsLayer, Any};

mod routes;
mod analytics;
mod listenbrainz;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any) // allow all origins (dev only)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::routes())
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("Server running on port 3001");

    axum::serve(listener, app).await.unwrap();
}