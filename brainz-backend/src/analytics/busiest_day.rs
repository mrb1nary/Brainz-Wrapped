use chrono::{DateTime, Utc, Local, TimeZone};
use std::collections::HashMap;

use super::Analytics;

#[derive(serde::Serialize)]
pub struct BusiestDay {
    pub date: String,   // RFC3339 timestamp
    pub listens: u32,
}

impl Analytics {

    pub fn busiest_day(&self) -> Option<BusiestDay> {

        let mut counts: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {

            // safely convert timestamp
            let utc = match DateTime::<Utc>::from_timestamp(listen.listened_at, 0) {
                Some(dt) => dt,
                None => continue, // skip invalid timestamps
            };

            let local = utc.with_timezone(&Local);

            // still group by date (not full timestamp)
            let date_key = local.date_naive().to_string();

            *counts.entry(date_key).or_insert(0) += 1;
        }

        // find busiest day
        let (date, listens) = counts
            .into_iter()
            .max_by_key(|(_, count)| *count)?;

        // convert that date into a proper timestamp (midnight local)
        let local_dt = Local
            .from_local_datetime(
                &chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
                    .ok()?
                    .and_hms_opt(0, 0, 0)?
            )
            .single()?;

        Some(BusiestDay {
            date: local_dt.to_rfc3339(),
            listens,
        })
    }

}