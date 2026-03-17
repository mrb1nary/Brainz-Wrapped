use chrono::{DateTime, Utc, Local, Datelike, Weekday};

use super::Analytics;

impl Analytics {

    pub fn weekday_distribution(&self) -> Vec<(String, u32)> {

        let mut counts = [0u32; 7];

        for listen in &self.listens {

            let utc = DateTime::<Utc>::from_timestamp(listen.listened_at, 0)
                .unwrap();

            let local = utc.with_timezone(&Local);

            let weekday = local.weekday().num_days_from_monday() as usize;

            counts[weekday] += 1;
        }

        let labels = [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ];

        labels
            .iter()
            .enumerate()
            .map(|(i, &day)| (day.to_string(), counts[i]))
            .collect()
    }
}