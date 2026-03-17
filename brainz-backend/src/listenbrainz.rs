use chrono::{Duration, Utc};
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListenResponse {
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub listens: Vec<Listen>,
}

#[derive(Debug, Deserialize)]
pub struct Listen {
    pub listened_at: i64,
    pub track_metadata: TrackMetadata,
}

#[derive(Debug, Deserialize)]
pub struct TrackMetadata {
    pub artist_name: String,
    pub track_name: String,
}

pub async fn fetch_last_year_listens(username: &str)->Result<Vec<Listen>, reqwest::Error>{
    // let client = Client::new();
    let client = Client::builder()
        .http1_only()
        .user_agent("brainz-wrapped/0.1")
        .build()?;
    let mut all_listens = Vec::new();


    let mut max_ts: Option<i64> = None;

    loop{
        let url = match max_ts {
            Some(ts) => format!(
                "https://api.listenbrainz.org/1/user/{}/listens?count=100&max_ts={}",
                username, ts
            ),
            None => format!(
                "https://api.listenbrainz.org/1/user/{}/listens?count=100",
                username
            ),
        };

        let res: ListenResponse = client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        if res.payload.listens.is_empty(){
            break;
        }
        let mut stop = false;

        for listen in res.payload.listens{
            if is_older_than_one_year(listen.listened_at){
                stop = true;
                break;
            }

            all_listens.push(listen);
        }

        if stop{
            break;
        }

        let oldest = all_listens.last().unwrap().listened_at;
        max_ts = Some(oldest-1);
    }

    Ok(all_listens)
}

fn is_older_than_one_year(timestamp: i64) -> bool {
    let now = Utc::now().timestamp();
    let one_year_ago = now - Duration::days(365).num_seconds();

    timestamp < one_year_ago
}

