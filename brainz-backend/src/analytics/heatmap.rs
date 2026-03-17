use std::collections::HashMap;
use chrono::{DateTime, Utc, Local};

use super::Analytics;

impl Analytics {

    pub fn heatmap(&self) -> HashMap<String, u32> {

        let mut map: HashMap<String, u32> = HashMap::new();

        for listen in &self.listens {

            // Convert timestamp → UTC
            let utc = DateTime::<Utc>::from_timestamp(listen.listened_at, 0)
                .unwrap();

            // Convert UTC → Local timezone
            let local = utc.with_timezone(&Local);

            // Extract local date
            let date = local.date_naive().to_string();

            *map.entry(date).or_insert(0) += 1;
        }

        map
    }

}