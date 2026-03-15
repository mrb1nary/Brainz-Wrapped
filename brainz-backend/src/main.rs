mod routes;
mod listenbrainz;
mod analytics;

use axum::Router;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::routes());
        // .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    println!("Server running on port 3001");

    axum::serve(listener, app).await.unwrap();
}