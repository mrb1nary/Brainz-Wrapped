use crate::listenbrainz::Listen;

pub mod heatmap;
pub mod hourly;
pub mod streaks;
pub mod busiest_day;

pub struct Analytics {
    listens: Vec<Listen>,
}

impl Analytics {
    pub fn new(listens: Vec<Listen>) -> Self {
        Self { listens }
    }

    pub fn listens(&self) -> &Vec<Listen> {
        &self.listens
    }
}