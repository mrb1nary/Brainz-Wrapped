use std::collections::HashMap;

use super::Analytics;

#[derive(serde::Serialize)]
pub struct ArtistStat {
    pub artist: String,
    pub listens: u32,
}

impl Analytics {

    pub fn top_artists(&self, limit: usize) -> Vec<ArtistStat> {

        let mut counts: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {

            let artist = listen .track_metadata .artist_name .to_lowercase();

            *counts.entry(artist.clone()).or_insert(0) += 1;
        }

        let mut artists: Vec<(String, u32)> = counts.into_iter().collect();

        artists.sort_by(|a, b| b.1.cmp(&a.1));

        artists
            .into_iter()
            .take(limit)
            .map(|(artist, listens)| ArtistStat { artist, listens })
            .collect()
    }

}