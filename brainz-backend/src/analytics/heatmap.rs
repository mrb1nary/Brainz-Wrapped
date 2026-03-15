use std::collections::HashMap;
use chrono::{DateTime, Utc};

use super::Analytics;

impl Analytics {

    pub fn heatmap(&self) -> HashMap<String, u32> {

        let mut map: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {

            let dt = DateTime::<Utc>::from_timestamp(listen.listened_at, 0)
                .unwrap();

            let date = dt.date_naive().to_string();

            *map.entry(date).or_insert(0) += 1;
        }

        map
    }

}