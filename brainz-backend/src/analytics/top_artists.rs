use std::collections::HashMap;

use futures::{stream, StreamExt};
use once_cell::sync::Lazy;
use serde::Deserialize;
use tokio::sync::Mutex;
use urlencoding::encode;

use super::Analytics;

static ARTIST_IMAGE_CACHE: Lazy<Mutex<HashMap<String, Option<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .user_agent("brainzwrapped/1.0 (your@email.com)")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .expect("Failed to build HTTP client")
});

#[derive(serde::Serialize, Clone)]
pub struct ArtistStat {
    pub artist: String,
    pub listens: u32,
    pub image: Option<String>,
}

impl Analytics {
    pub async fn top_artists(&self, limit: usize) -> Vec<ArtistStat> {
        let mut counts: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {
            let normalized = normalize_artist(&listen.track_metadata.artist_name);
            *counts.entry(normalized).or_insert(0) += 1;
        }

        let mut artists: Vec<(String, u32)> = counts.into_iter().collect();
        artists.sort_by(|a, b| b.1.cmp(&a.1));

        // limit concurrency to avoid rate limiting
        stream::iter(artists.into_iter().take(limit))
            .map(|(artist, listens)| async move {
                println!("[TOP_ARTIST] Processing: {}", artist);

                let image = get_artist_image(&artist).await;

                println!(
                    "[TOP_ARTIST] Result: {} -> image: {:?}",
                    artist, image
                );

                ArtistStat {
                    artist,
                    listens,
                    image,
                }
            })
            .buffer_unordered(5) // control concurrency
            .collect()
            .await
    }
}

fn normalize_artist(raw: &str) -> String {
    raw.split(',')
        .next()
        .unwrap_or(raw)
        .trim()
        .to_lowercase()
}

/* ---------------- MUSICBRAINZ SEARCH ---------------- */

#[derive(Deserialize)]
struct MBArtistResponse {
    artists: Vec<MBArtist>,
}

#[derive(Deserialize)]
struct MBArtist {
    id: String,
}

async fn get_artist_mbid(name: &str) -> Option<String> {
    let url = format!(
        "https://musicbrainz.org/ws/2/artist/?query=artist:{}&limit=1&fmt=json",
        encode(name)
    );

    println!("[MB SEARCH] {}", name);

    let res = HTTP_CLIENT.get(&url).send().await.ok()?;
    let json: MBArtistResponse = res.json().await.ok()?;

    let mbid = json.artists.first()?.id.clone();

    println!("[MB SEARCH] Result: {}", mbid);

    Some(mbid)
}

/* ---------------- MUSICBRAINZ LOOKUP ---------------- */

#[derive(Deserialize)]
struct MBArtistLookup {
    relations: Option<Vec<Relation>>,
}

#[derive(Deserialize)]
struct Relation {
    #[serde(rename = "type")]
    rel_type: String,
    url: RelationUrl,
}

#[derive(Deserialize)]
struct RelationUrl {
    resource: String,
}

async fn get_wikidata_id_from_mbid(mbid: &str) -> Option<String> {
    let url = format!(
        "https://musicbrainz.org/ws/2/artist/{}?inc=url-rels&fmt=json",
        mbid
    );

    println!("[MB LOOKUP] {}", mbid);

    let res = HTTP_CLIENT.get(&url).send().await.ok()?;
    let json: MBArtistLookup = res.json().await.ok()?;

    let relations = json.relations?;

    for rel in relations {
        if rel.rel_type == "wikidata" {
            let qid = rel.url.resource.split('/').last()?.to_string();
            println!("[MB] Wikidata ID: {}", qid);
            return Some(qid);
        }
    }

    println!("[MB] No Wikidata found");
    None
}

/* ---------------- WIKIDATA ---------------- */

#[derive(Deserialize)]
struct WikidataResponse {
    entities: HashMap<String, Entity>,
}

#[derive(Deserialize)]
struct Entity {
    claims: HashMap<String, Vec<Claim>>,
}

#[derive(Deserialize)]
struct Claim {
    mainsnak: MainSnak,
}

#[derive(Deserialize)]
struct MainSnak {
    datavalue: Option<DataValue>,
}

#[derive(Deserialize)]
struct DataValue {
    value: serde_json::Value,
}

async fn get_wikidata_image(qid: &str) -> Option<String> {
    let url = format!(
        "https://www.wikidata.org/wiki/Special:EntityData/{}.json",
        qid
    );

    println!("[WIKIDATA] Fetching {}", qid);

    let res = HTTP_CLIENT.get(&url).send().await.ok()?;
    let json: WikidataResponse = res.json().await.ok()?;

    let entity = json.entities.get(qid)?;

    let claims = entity.claims.get("P18")?;

    let filename = claims
        .first()?
        .mainsnak
        .datavalue
        .as_ref()?
        .value
        .as_str()?;

    println!("[WIKIDATA] File: {}", filename);

    let encoded = filename.replace(" ", "_");

    Some(format!(
        "https://commons.wikimedia.org/wiki/Special:FilePath/{}",
        encoded
    ))
}

/* ---------------- MAIN PIPELINE ---------------- */

async fn get_artist_image(name: &str) -> Option<String> {
    let key = name.to_lowercase();

    // cache (including None)
    if let Some(cached) = ARTIST_IMAGE_CACHE.lock().await.get(&key).cloned() {
        println!("[CACHE HIT] {} -> {:?}", key, cached);
        return cached;
    }

    println!("[CACHE MISS] {}", key);

    let result = async {
        let mbid = get_artist_mbid(name).await?;
        let qid = get_wikidata_id_from_mbid(&mbid).await?;
        get_wikidata_image(&qid).await
    }
        .await;

    // cache both success + failure
    ARTIST_IMAGE_CACHE
        .lock()
        .await
        .insert(key.clone(), result.clone());

    result
}
