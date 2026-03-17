use std::collections::HashMap;

use axum::{
    extract::Path,
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::analytics::Analytics;
use crate::analytics::busiest_day::BusiestDay;
use crate::analytics::listening_age::ListeningAge;
use crate::analytics::sessions::SessionStats;
use crate::analytics::streaks::StreakStats;
use crate::analytics::top_artists::ArtistStat;
use crate::analytics::top_tracks::TrackStat;
use crate::analytics::stats::StatsResponse;

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
        .route("/listening-age/{username}", get(listening_age_handler))
        .route("/stats/{username}", get(stats))
}

async fn health() -> &'static str {
    println!("Server is good!");
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

    // FIX: await async function
    let result = analytics.top_artists(10).await;

    Ok(Json(result))
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

async fn stats(
    Path(username): Path<String>,
) -> Result<Json<StatsResponse>, StatusCode> {

    println!("[ROUTE] stats HIT {}", username);

    let listens = match fetch_last_year_listens(&username).await {
        Ok(l) => {
            println!("[FETCH] success, listens: {}", l.len());
            l
        }
        Err(e) => {
            println!("[FETCH][ERROR] {:?}", e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    println!("[DEBUG] creating analytics");

    let analytics = Analytics::new(listens);

    println!("[DEBUG] calling full_stats");

    let result = analytics.full_stats().await;

    println!("[DEBUG] full_stats done");

    Ok(Json(result))
}

async fn listening_age_handler(
    Path(username): Path<String>,
) -> Json<Vec<ListeningAge>> {
    let listens = fetch_last_year_listens(&username).await.unwrap();

    let analytics = Analytics::new(listens);

    let data = analytics.listening_age().await;

    Json(data)
}