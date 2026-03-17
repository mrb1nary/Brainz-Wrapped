use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::analytics::Analytics;
use crate::analytics::busiest_day::BusiestDay;
use crate::analytics::sessions::SessionStats;
use crate::analytics::streaks::StreakStats;
use crate::analytics::top_artists::ArtistStat;
use crate::analytics::top_tracks::TrackStat;
use crate::listenbrainz::fetch_last_year_listens;

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/heatmap/{username}", get(heatmap))
        .route("/hourly/{username}", get(hourly))
        .route("/streaks/{username}", get(streaks))
        .route("/busiest-day/{username}", get(busiest_day))
        .route("/top-artists/{username}", get(top_artists))
        .route("/top-tracks/{username}", get(top_tracks))
        .route("/sessions/{username}", get(sessions))
        .route("/weekday/{username}", get(weekday))
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

async fn hourly(
    Path(username): Path<String>,
) -> Result<Json<HashMap<u32, u32>>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.listens_per_hour()))
}

async fn streaks(
    Path(username): Path<String>,
) -> Result<Json<StreakStats>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.streaks()))
}

async fn busiest_day(
    Path(username): Path<String>,
) -> Result<Json<BusiestDay>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    analytics
        .busiest_day()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn top_artists(
    Path(username): Path<String>,
) -> Result<Json<Vec<ArtistStat>>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.top_artists(10)))
}

async fn top_tracks(
    Path(username): Path<String>,
) -> Result<Json<Vec<TrackStat>>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.top_tracks(10)))
}

async fn sessions(
    Path(username): Path<String>,
) -> Result<Json<SessionStats>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.listening_sessions()))
}

async fn weekday(
    Path(username): Path<String>,
) -> Result<Json<Vec<(String, u32)>>, StatusCode> {

    let listens = fetch_last_year_listens(&username)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let analytics = Analytics::new(listens);

    Ok(Json(analytics.weekday_distribution()))
}