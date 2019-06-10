use std::thread;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use super::time::{Moment, Interval};
use super::Configuration;

#[derive(Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: u64,
    pub events: Vec<Interval>
}

impl Schedule {
    pub fn in_interval(&self, t: Moment) -> bool {
        for interval in self.events.iter() {
            if interval.contains(t) {
                return true;
            }
        }

        return false;
    }
}

pub fn scheduler(mut config: Configuration, recv: Receiver<Configuration>, tickrate: u64) {

    let mut open = false;

    loop {

        // Receive updated config.
        match recv.try_recv() {
            Ok(updated_config) => {
                config = updated_config;

                if open {
                    // Handle interruption. Maybe close the valve?
                    println!("Deactivate GPIO.");
                }

                println!("Configuration was updated.");
            }
            _ => {}
        }

        let now = Moment::now();
        
        if !config.enabled {

            let in_interval = config.schedule.in_interval(now);
            if in_interval && !open {
                open = true;

                // Do GPIO stuff here.
                println!("Activated GPIO");
            } else if !in_interval && open {
                open = false;

                // Do GPIO stuff here.
                println!("Deactivated GPIO");
            }

            println!("Open: {}", open);

        } else {
            println!("Sprinkler is disabled, zZzzZ...");
        }

        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_millis() as u64 % tickrate;
        thread::sleep(Duration::from_millis(tickrate - t));
    }
}