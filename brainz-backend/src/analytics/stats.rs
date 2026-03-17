use serde::Serialize;

use crate::analytics::top_artists::ArtistStat;
use crate::analytics::top_tracks::TrackStat;
use crate::analytics::sessions::SessionStats;
use crate::analytics::streaks::StreakStats;
use crate::analytics::busiest_day::BusiestDay;

#[derive(Serialize)]
pub struct StatsResponse {
    pub top_artists: Vec<ArtistStat>,
    pub top_tracks: Vec<TrackStat>,
    pub sessions: SessionStats,
    pub streaks: StreakStats,
    pub busiest_day: Option<BusiestDay>,
    pub hourly: std::collections::HashMap<u32, u32>,
    pub weekday: Vec<(String, u32)>,
}