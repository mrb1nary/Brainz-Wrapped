use serde::Serialize;
use std::collections::HashMap;

use crate::analytics::top_artists::ArtistStat;
use crate::analytics::top_tracks::TrackStat;
use crate::analytics::sessions::SessionStats;
use crate::analytics::streaks::StreakStats;
use crate::analytics::busiest_day::BusiestDay;

use super::Analytics;

#[derive(Serialize)]
pub struct StatsResponse {
    pub top_artists: Vec<ArtistStat>,
    pub top_tracks: Vec<TrackStat>,
    pub sessions: SessionStats,
    pub streaks: StreakStats,
    pub busiest_day: Option<BusiestDay>,
    pub hourly: HashMap<u32, u32>,
    pub weekday: Vec<(String, u32)>,
    pub heatmap: HashMap<String, u32>,
}

impl Analytics {
    /// Full stats (async because artist images are fetched)
    pub async fn full_stats(&self) -> StatsResponse {
        println!("[STATS] full_stats called");

        let top_artists = self.top_artists(10).await;

        println!(
            "[STATS] top_artists computed: {} artists",
            top_artists.len()
        );

        StatsResponse {
            top_artists,
            top_tracks: self.top_tracks(10),
            sessions: self.listening_sessions(),
            streaks: self.streaks(),
            busiest_day: self.busiest_day(),
            hourly: self.listens_per_hour(),
            weekday: self.weekday_distribution(),
            heatmap: self.heatmap(),
        }
    }
}