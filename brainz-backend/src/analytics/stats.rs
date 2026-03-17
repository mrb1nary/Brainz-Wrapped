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

    pub fn full_stats(&self) -> StatsResponse {

        StatsResponse {
            top_artists: self.top_artists(10),
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