use chrono::{DateTime, Duration, Local, Utc};

use super::Analytics;

#[derive(serde::Serialize)]
pub struct SessionStats {
    pub total_sessions: u32,
    pub average_session_minutes: u32,
    pub longest_session_minutes: u32,
}

impl Analytics {

    pub fn listening_sessions(&self) -> SessionStats {

        if self.listens.is_empty() {
            return SessionStats {
                total_sessions: 0,
                average_session_minutes: 0,
                longest_session_minutes: 0,
            };
        }

        // Convert timestamps to local datetimes
        let mut times: Vec<_> = self
            .listens
            .iter()
            .map(|l| {
                DateTime::<Utc>::from_timestamp(l.listened_at, 0)
                    .unwrap()
                    .with_timezone(&Local)
            })
            .collect();

        // Sort chronologically
        times.sort();

        let mut sessions: Vec<Duration> = Vec::new();

        let mut session_start = times[0];
        let mut last = times[0];

        for time in times.iter().skip(1) {

            let gap = *time - last;

            if gap > Duration::minutes(30) {

                // end previous session
                sessions.push(last - session_start);

                // start new session
                session_start = *time;
            }

            last = *time;
        }

        // push final session
        sessions.push(last - session_start);

        let total_sessions = sessions.len() as u32;

        let total_minutes: i64 =
            sessions.iter().map(|d| d.num_minutes()).sum();

        let longest = sessions
            .iter()
            .map(|d| d.num_minutes())
            .max()
            .unwrap_or(0);

        SessionStats {
            total_sessions,
            average_session_minutes: (total_minutes / total_sessions as i64) as u32,
            longest_session_minutes: longest as u32,
        }
    }
}