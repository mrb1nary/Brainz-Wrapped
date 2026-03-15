use chrono::{DateTime, Utc, Local, NaiveDate};
use std::collections::HashSet;

use super::Analytics;

#[derive(serde::Serialize)]
pub struct StreakStats {
    pub current_streak: u32,
    pub longest_streak: u32,
}

impl Analytics {

    pub fn streaks(&self) -> StreakStats {

        // 1) Collect unique listening days (in local timezone)
        let mut days_set: HashSet<NaiveDate> = HashSet::new();

        for listen in &self.listens {
            let utc = DateTime::<Utc>::from_timestamp(listen.listened_at, 0).unwrap();
            let local = utc.with_timezone(&Local);

            days_set.insert(local.date_naive());
        }

        // 2) Sort the days
        let mut days: Vec<NaiveDate> = days_set.into_iter().collect();
        days.sort();

        // 3) Compute longest streak
        let mut longest = 0;
        let mut current_run = 0;
        let mut prev: Option<NaiveDate> = None;

        for day in &days {

            if let Some(prev_day) = prev {

                if *day == prev_day.succ_opt().unwrap() {
                    current_run += 1;
                } else {
                    current_run = 1;
                }

            } else {
                current_run = 1;
            }

            longest = longest.max(current_run);
            prev = Some(*day);
        }

        // 4) Compute current streak (ending today or yesterday)
        let today = chrono::Local::now().date_naive();
        let mut current_streak = 0;

        let mut check_day = today;

        loop {
            if days.contains(&check_day) {
                current_streak += 1;
                check_day = check_day.pred_opt().unwrap();
            } else {
                break;
            }
        }

        StreakStats {
            current_streak,
            longest_streak: longest,
        }
    }
}