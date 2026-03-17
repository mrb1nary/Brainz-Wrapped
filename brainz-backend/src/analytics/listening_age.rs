use std::collections::HashMap;

use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use serde::Deserialize;
use tokio::sync::Mutex;
use urlencoding::encode;

use super::Analytics;

static TRACK_YEAR_CACHE: Lazy<Mutex<HashMap<String, Option<u32>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("brainzwrapped/1.0 (your@email.com)")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client")
});

/* ---------------- RESPONSE STRUCT ---------------- */

#[derive(serde::Serialize)]
pub struct ListeningAge {
    pub decade: String,
    pub listens: u32,
}

/* ---------------- MUSICBRAINZ SEARCH ---------------- */

#[derive(Deserialize)]
struct RecordingSearchResponse {
    recordings: Vec<Recording>,
}

#[derive(Deserialize)]
struct Recording {
    #[serde(rename = "first-release-date")]
    first_release_date: Option<String>,
}

/* ---------------- FETCH YEAR ---------------- */

async fn get_track_year(artist: &str, track: &str) -> Option<u32> {
    let key = format!(
        "{}::{}",
        artist.to_lowercase(),
        track.to_lowercase()
    );

    // cache hit
    if let Some(cached) = TRACK_YEAR_CACHE.lock().await.get(&key).cloned() {
        println!("[CACHE HIT] {}", key);
        return cached;
    }

    println!("[FETCH YEAR] {} - {}", artist, track);

    let url = format!(
        "https://musicbrainz.org/ws/2/recording/?query=artist:{} AND recording:{}&fmt=json&limit=1",
        encode(artist),
        encode(track)
    );

    let res = match HTTP_CLIENT.get(&url).send().await {
        Ok(r) => r,
        Err(_) => {
            TRACK_YEAR_CACHE.lock().await.insert(key, None);
            return None;
        }
    };

    let json: RecordingSearchResponse = match res.json().await {
        Ok(j) => j,
        Err(_) => {
            TRACK_YEAR_CACHE.lock().await.insert(key, None);
            return None;
        }
    };

    let year = json
        .recordings
        .first()
        .and_then(|r| r.first_release_date.as_ref())
        .and_then(|d| d.split('-').next())
        .and_then(|y| y.parse::<u32>().ok());

    println!("[YEAR] {:?} -> {:?}", key, year);

    TRACK_YEAR_CACHE.lock().await.insert(key, year);

    year
}

/* ---------------- MAIN ANALYTICS ---------------- */

impl Analytics {
    pub async fn listening_age(&self) -> Vec<ListeningAge> {
        // Step 1: collect unique tracks
        let mut unique = std::collections::HashSet::new();

        for listen in &self.listens {
            unique.insert((
                listen.track_metadata.artist_name.clone(),
                listen.track_metadata.track_name.clone(),
            ));
        }

        println!("[DEDUP] unique tracks: {}", unique.len());

        // Step 2: fetch years for unique tracks
        let track_years: HashMap<(String, String), Option<u32>> =
            stream::iter(unique.into_iter())
                .map(|(artist, track)| async move {
                    let year = get_track_year(&artist, &track).await;
                    ((artist, track), year)
                })
                .buffer_unordered(3)
                .collect()
                .await;

        // Step 3: aggregate using original listens
        let mut counts: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {
            let key = (
                listen.track_metadata.artist_name.clone(),
                listen.track_metadata.track_name.clone(),
            );

            if let Some(Some(year)) = track_years.get(&key) {
                let decade = format!("{}s", (year / 10) * 10);
                *counts.entry(decade).or_insert(0) += 1;
            }
        }

        // Step 4: format result
        let mut result: Vec<ListeningAge> = counts
            .into_iter()
            .map(|(decade, listens)| ListeningAge { decade, listens })
            .collect();

        result.sort_by(|a, b| b.listens.cmp(&a.listens));

        result
    }
}