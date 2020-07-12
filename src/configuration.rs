use std::io;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

use super::schedule::Schedule;
use super::time::{Interval, Moment};

#[derive(Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub overwrite: bool,
    pub schedule: Schedule
}

impl Configuration {
    pub fn default() -> Self {
        Configuration {
            overwrite: false,
            schedule: Schedule {
                id: 17,
                events: vec![
                    Interval::new(Moment::new(16, 00, 00), Moment::new(16, 00, 05)),
                    Interval::new(Moment::new(16, 01, 00), Moment::new(16, 01, 10)),
                    Interval::new(Moment::new(20, 40, 00), Moment::new(20, 40, 10)),
                ],
            },
        }
    }

    pub fn load_from(path: &str) -> Self {
        let mut file = match File::open(path) {
            Ok(file) => file,
            _ => {
                println!("No configuration file. Creating with defaults.");

                let mut file =
                    File::create(path).expect("Unable to create configuration file (config.json).");
                file.write_all(
                    serde_json::to_string_pretty(&Configuration::default())
                        .unwrap()
                        .as_bytes(),
                )
                .expect("Failed to write configuration file.");

                File::open("./config.json").expect("Unable to read config file.")
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read configuration file (config.json).");

        match serde_json::from_str(&contents) {
            Ok(c) => c,
            Err(msg) => panic!("Failed to parse configuration file (config.json), make sure it is correctly formatted. Error: {}", msg)
        }
    }

    pub fn save_to(&self, path: &str) -> Result<(), io::Error> {
        match File::create(path) {
            Ok(mut file) => file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes()),
            Err(e) => Err(e)
        }
    }
}