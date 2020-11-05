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

pub fn get_regions(wrap_state: Data<Arc<Mutex<AppState>>>, _req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_regions() {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}

pub fn get_departments(wrap_state: Data<Arc<Mutex<AppState>>>, _req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_departments() {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}

pub fn get_national_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    _req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_national_index() {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}

pub fn get_regional_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("region") {
        Some(region) => match domain.get_regional_index(region.to_string()) {
            Ok(entry) => HttpResponse::Ok().json(entry),
            Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}

pub fn get_departmental_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("dept") {
        Some(dept) => match domain.get_departmental_index(dept.to_string()) {
            Ok(entry) => HttpResponse::Ok().json(entry),
            Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}

pub fn get_district_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("iriscode") {
        Some(iriscode) => match domain.get_district_index(iriscode.to_string()) {
            Ok(entry) => HttpResponse::Ok().json(entry),
            Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}
