use crate::state::AppState;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Everything's fine.")
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

pub fn entries_get_all(wrap_state: Data<Arc<Mutex<AppState>>>, _req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_all() {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}
