use std::collections::HashMap;
use chrono::{DateTime, Utc, Local, Timelike};

use super::Analytics;

impl Analytics {

    pub fn listens_per_hour(&self) -> HashMap<u32, u32> {

        let mut hours: HashMap<u32, u32> =
            (0..24).map(|h| (h, 0)).collect();

        for listen in &self.listens {

            let utc = DateTime::<Utc>::from_timestamp(listen.listened_at, 0)
                .unwrap();

            let local = utc.with_timezone(&Local);

            let hour = local.hour();

            *hours.entry(hour).or_insert(0) += 1;
        }

        hours
    }

}