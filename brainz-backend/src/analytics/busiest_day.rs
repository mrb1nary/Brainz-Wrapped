use chrono::{DateTime, Utc, Local};
use std::collections::HashMap;

use super::Analytics;

#[derive(serde::Serialize)]
pub struct BusiestDay {
    pub date: String,
    pub listens: u32,
}

impl Analytics {

    pub fn busiest_day(&self) -> Option<BusiestDay> {

        let mut counts: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {

            let utc = DateTime::<Utc>::from_timestamp(listen.listened_at, 0)
                .unwrap();

            let local = utc.with_timezone(&Local);

            let date = local.date_naive().to_string();

            *counts.entry(date).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(date, listens)| BusiestDay { date, listens })
    }

}