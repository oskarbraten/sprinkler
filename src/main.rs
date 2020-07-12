
use std::io;
use std::thread;
use std::sync::{mpsc};

use actix_web::{HttpServer, App, web, middleware, HttpRequest};
use actix_web::web::{Data, Json};
use actix_files as fs;

mod time;
mod schedule;
mod configuration;

use configuration::Configuration;

#[cfg(debug_assertions)]
const ADDRESS: &str = "0.0.0.0:8080";

#[cfg(not(debug_assertions))]
const ADDRESS: &str = "0.0.0.0:80";

const TICKRATE: u64 = 1000;

fn get_config(_req: HttpRequest) -> io::Result<fs::NamedFile> {
    fs::NamedFile::open("config.json")
}

fn put_config(mut body: Json<Configuration>, dtx: Data<mpsc::Sender<Configuration>>) -> Json<Configuration> {
    body.schedule.sort(); // sort schedule before updating.
    match body.save_to("./config.json") {
        Err(_) => {
            panic!("Unable to save to configuration file.");
        },
        _ => {}
    };

    dtx.send(body.clone()).unwrap();
    Json(body.clone()) // send back the updated configuration.
}

fn main() -> io::Result<()> {
    // Setup mpsc-channel.
    let (tx, rx) = mpsc::channel::<Configuration>();

    // Start irrigation system:
    {
        let config = Configuration::load_from("./config.json");
        thread::spawn(move || schedule::scheduler(config, rx, TICKRATE));
    }

    // Start webserver:
    println!("Starting server on: {}", ADDRESS);
    println!("URL: http://{}", ADDRESS);

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = actix_rt::System::new("Sprinkler");

    HttpServer::new(move || {
        App::new()
            .data(tx.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/configuration")
                    .route(web::get().to(get_config))
                    .route(web::put().to(put_config)),
            )
            .service(fs::Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind(ADDRESS)
    .expect("Unable to start HTTP-server, you may have to run it as root.")
    .start();

    sys.run()
}