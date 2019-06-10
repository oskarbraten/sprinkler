
use chrono::prelude::*;
use std::sync::mpsc::Receiver;

use std::thread;
use std::time::Duration;

use super::configuration::Configuration;

pub fn timer(mut config: Configuration, recv: Receiver<Configuration>, tickrate: u64) {

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

        let now = Local::now().time();
        
        if !config.disabled {

            let in_activation = config.in_activation(&now);
            if in_activation && !open {
                open = true;

                // Do GPIO stuff here.
                println!("Activated GPIO");
            } else if !in_activation && open {
                open = false;

                // Do GPIO stuff here.
                println!("Deactivated GPIO");
            }

            println!("Open: {}", open);

        } else {
            println!("Sprinkler is disabled, zZzzZ...");
        }

        let t = Local::now().timestamp_subsec_millis() as u64 % tickrate;
        thread::sleep(Duration::from_millis(tickrate - t));
    }
}