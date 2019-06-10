
use chrono::prelude::*;
use std::time::Duration;
use chrono::Duration as ChronoDuration;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Activation {
    pub time: NaiveTime,
    pub duration: Duration,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub disabled: bool,
    pub activations: Vec<Activation>,
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            disabled: true,
            activations: vec![
                Activation {
                    time: NaiveTime::from_hms(8, 0, 0),
                    duration: Duration::from_secs(60),
                },
                Activation {
                    time: NaiveTime::from_hms(20, 0, 0),
                    duration: Duration::from_secs(60),
                },
            ],
        }
    }

    pub fn find_next(&mut self, time: &NaiveTime) -> Option<Activation> {

        self.activations.sort_by(|a, b| {
            let delta_b = b.time.cmp(&time);
            let delta_a = a.time.cmp(&time);

            delta_b.cmp(&delta_a)
        });

        return match self.activations.get(0) {
            Some(activation) => Some(activation.clone()),
            None => None
        };
    }

    pub fn in_activation(&self, time: &NaiveTime) -> bool {
        for activation in self.activations.clone() {
            let low = activation.time;
            let high = activation.time + ChronoDuration::from_std(activation.duration).unwrap(); // TODO: handle fail.

            if (time > &low) && (time < &high) {
                return true;
            }
        }

        return false;
    }
}