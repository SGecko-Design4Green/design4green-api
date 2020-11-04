//pub mod actix_middle;
//pub mod cache_static;
pub mod configuration;
pub mod controller;
pub mod state;

use crate::configuration::Configuration;
use crate::controller::*;
use crate::state::AppState;
use actix_files as fs;
use actix_web::{middleware, web, web::Data, App, HttpServer};
use std::sync::{Arc, Mutex};
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("[DESIGN4GREEN API {}]", env!("CARGO_PKG_VERSION"));

    //Configuration init.
    Configuration::load();

    //Set the IP:PORT to be served.
    let addr = Configuration::get_served_addr();
    print!("--> Started on ");
    println!("http://{}", addr);

    //Logger service initialization
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //Define a global state for all the Actix-Worker
    let app_state = Arc::new(Mutex::new(AppState::new()));

    //Start server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/_", web::get().to(healthcheck))
                    .route("/index", web::get().to(entries_get_all)),
            )
            //STATIC CONF
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Cache-Control", "public, max-age=604800, immutable"),
            )
            .service(web::scope("/").configure(get_static_files_configuration))
    })
    .bind(&addr)
    .expect("Cannot bind to address.")
    .keep_alive(Configuration::get_keep_alive())
    .shutdown_timeout(Configuration::get_shutdown_time_out())
    .workers(Configuration::get_workers_number())
    .run()
    .await
}

fn get_static_files_configuration(cfg: &mut web::ServiceConfig) {
    //----------------------------------------------------------
    //___STATIC_FILES___
    //----------------------------------------------------------
    cfg.service(
        fs::Files::new("", "./static")
            .prefer_utf8(true)
            .index_file("index.html"),
    );
}
