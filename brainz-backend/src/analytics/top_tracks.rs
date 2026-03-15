use std::collections::HashMap;

use super::Analytics;

#[derive(serde::Serialize)]
pub struct TrackStat {
    pub track: String,
    pub artist: String,
    pub plays: u32,
}

impl Analytics {

    pub fn top_tracks(&self, limit: usize) -> Vec<TrackStat> {

        let mut counts: HashMap<(String, String), u32> = HashMap::new();

        for listen in &self.listens {

            let artist = listen
                .track_metadata
                .artist_name
                .trim()
                .to_lowercase();

            let track = listen
                .track_metadata
                .track_name
                .trim()
                .to_lowercase();

            *counts.entry((artist, track)).or_insert(0) += 1;
        }

        let mut tracks: Vec<((String, String), u32)> =
            counts.into_iter().collect();

        tracks.sort_by(|a, b| b.1.cmp(&a.1));

        tracks
            .into_iter()
            .take(limit)
            .map(|((artist, track), plays)| TrackStat {
                artist,
                track,
                plays,
            })
            .collect()
    }

}