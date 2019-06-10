use serde::{Deserialize, Serialize};

use super::time::{Moment, Interval};
use super::schedule::Schedule;

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub enabled: bool,
    pub schedule: Schedule,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            enabled: false,
            schedule: Schedule {
                id: 0,
                events: vec![
                    Interval::new(Moment::new(16, 00, 00), Moment::new(16, 00, 05)),
                    Interval::new(Moment::new(16, 01, 00), Moment::new(16, 01, 10)),
                ]
            }
        }
    }
}