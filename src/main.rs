
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

use actix_web::{HttpServer, App, web, middleware, HttpRequest};
use actix_web::web::{Data, Json};

mod configuration;
mod timer;

use configuration::Configuration;

const ADDRESS: &str = "127.0.0.1:8088";
const TICKRATE: u64 = 1000;

fn index() -> &'static str {
    "Hello, sprinkler!"
}

fn get_config(_req: HttpRequest, data: Data<Arc<Mutex<Configuration>>>) -> Json<Configuration> {
    Json(data.lock().unwrap().clone())
}

fn put_config(body: Json<Configuration>, data: Data<Arc<Mutex<Configuration>>>, dtx: Data<mpsc::Sender<Configuration>>) -> Json<Configuration> {
    let mut config = data.lock().unwrap();
    *config = body.into_inner(); // overwrite existing configuration.

    dtx.send(config.clone()).unwrap();

    // Save updated configuration:
    match File::create("./config.json") {
        Ok(mut file) => {
            file.write_all(serde_json::to_string_pretty(&config.clone()).unwrap().as_bytes()).expect("Failed to write configuration file.");
        },
        _ => {
            println!("Unable to open configuration file.");
        }
    };

    Json(config.clone()) // send back the updated configuration.
}

fn main() -> io::Result<()> {
    let mut file = match File::open("./config.json") {
        Ok(file) => file,
        _ => {
            let mut file = File::create("./config.json").expect("Unable to create configuration file (config.json).");
            file.write_all(serde_json::to_string_pretty(&Configuration::default()).unwrap().as_bytes()).expect("Failed to write configuration file.");
            
            File::open("./config.json").expect("Unable to read config file.")
        }
    };

    let config: Configuration = {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read configuration file (config.json).");

        match serde_json::from_str(&contents) {
            Ok(c) => c,
            Err(msg) => panic!("Failed to parse configuration file (config.json), make sure it is correctly formatted. Error: {}", msg)
        }
    };
    
    // Setup mpsc-channel.
    let (tx, rx) = mpsc::channel::<Configuration>();

    // Start irrigation system:
    {
        let c = config.clone();
        thread::spawn(move || timer::timer(c, rx, TICKRATE));
    }

    // Start webserver:
    println!("Server running on: {}", ADDRESS);
    println!("URL: http://{}", ADDRESS);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix_rt::System::new("Sprinkler");

    HttpServer::new(move || {
        App::new()
            .data(Arc::new(Mutex::new(config.clone())))
            .data(tx.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(
                web::resource("/configuration")
                    .route(web::get().to(get_config))
                    .route(web::put().to(put_config)),
            )
    })
    .bind(ADDRESS)
    .unwrap()
    .start();

    sys.run()
}