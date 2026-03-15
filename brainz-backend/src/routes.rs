use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::analytics::Analytics;
use crate::listenbrainz::fetch_last_year_listens;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/heatmap/{username}", get(heatmap))
}

async fn health() -> &'static str {
    "Server healthy"
}

async fn heatmap(
    Path(username): Path<String>,
) -> Result<Json<HashMap<String, u32>>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.heatmap()))
}