use std::thread;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(debug_assertions))]
use rppal::gpio::Gpio;

#[cfg(not(debug_assertions))]
use rppal::system::DeviceInfo;

use serde::{Deserialize, Serialize};

use super::time::{Moment, Interval};
use super::Configuration;

#[derive(Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub id: u8,
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

    pub fn sort(&mut self) {
        self.events.sort();
    }
}

pub fn scheduler(mut config: Configuration, recv: Receiver<Configuration>, tickrate: u64) {

    #[cfg(not(debug_assertions))]
    println!("Sprinkler scheduler running on device: {}.", DeviceInfo::new().expect("Unable to get RPI device info.").model());

    #[cfg(not(debug_assertions))]
    let gpio = Gpio::new().expect("Unable to construct GPIO.");

    #[cfg(not(debug_assertions))]
    let mut pin = gpio.get(config.schedule.id).expect("Unable to get pin.").into_output();

    let mut open = false;

    loop {

        // Receive updated config.
        match recv.try_recv() {
            Ok(updated_config) => {
                config = updated_config;

                if open {
                    // Configuration was updated. Schedule may have changed, deactivate pin.

                    #[cfg(not(debug_assertions))]
                    pin.set_low();
                    
                    open = false;
                    println!("Deactivated pin: {}", config.schedule.id);
                }

                println!("Configuration was updated.");
            }
            _ => {}
        }

        let now = Moment::now();
        
        if config.enabled {

            let in_interval = config.schedule.in_interval(now);
            if in_interval && !open {
                open = true;

                #[cfg(not(debug_assertions))]
                pin.set_high();
                
                println!("Activated pin: {}", config.schedule.id);

            } else if !in_interval && open {
                open = false;

                #[cfg(not(debug_assertions))]
                pin.set_low();

                println!("Deactivated pin: {}", config.schedule.id);
            }

            #[cfg(debug_assertions)]
            println!("Open: {}", open);

        } else {
            #[cfg(debug_assertions)]
            println!("Sprinkler is disabled, zZzzZ...");
        }

        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_millis() as u64 % tickrate;
        thread::sleep(Duration::from_millis(tickrate - t));
    }
}