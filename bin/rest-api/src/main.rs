#[macro_use]
extern crate serde_derive;

pub mod configuration;
pub mod controller;
pub mod state;

use crate::configuration::Configuration;
use crate::controller::*;
use crate::state::AppState;
use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
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

    // let cors = Cors::new().supports_credentials();

    //Start server
    HttpServer::new(move || {
        App::new()
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Access-Control-Allow-Origin", "*")
                    .header("Access-Control-Allow-Methods", "*")
                    .header("Access-Control-Allow-Headers", "*"),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            //STATIC CONF
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("Cache-Control", "public, max-age=604800, immutable"),
            )
            .data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/_", web::get().to(healthcheck))
                    .route("/regions", web::get().to(get_regions))
                    .route("/departments", web::get().to(get_departments))
                    .route("/cities", web::get().to(get_cities))
                    .route("/cities/search", web::get().to(search_cities))
                    .route("/index/national", web::get().to(get_national_index))
                    .route(
                        "/index/regional/{region}",
                        web::get().to(get_regional_index),
                    )
                    .route(
                        "/index/regional/{region}/in",
                        web::get().to(get_in_regional_index),
                    )
                    .route("/index/regional", web::get().to(get_all_regional_index))
                    .route(
                        "/index/departmental/{dept}",
                        web::get().to(get_departmental_index),
                    )
                    .route(
                        "/index/departmental/{dept}/in",
                        web::get().to(get_in_departmental_index),
                    )
                    .route("/index/city/{code_insee}", web::get().to(get_city_index))
                    .route(
                        "/index/city/{code_insee}/districts",
                        web::get().to(get_city_districts_index),
                    )
                    .route(
                        "/index/districts/{iriscode}",
                        web::get().to(get_district_index),
                    )
                    .route("/index", web::get().to(entries_get_all)),
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
