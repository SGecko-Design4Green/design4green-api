use crate::state::AppState;
use actix_web::web::{Data, Query};
use actix_web::{web, HttpRequest, HttpResponse};
use std::sync::{Arc, Mutex};
use urlencoding::decode;

#[derive(Deserialize)]
pub struct SearchQuery {
    department: Option<String>,
    q: Option<String>,
}

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Everything's fine.")
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

#[derive(Deserialize)]
pub struct PageParam {
    page: i32
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

pub fn get_cities(wrap_state: Data<Arc<Mutex<AppState>>>, _req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_cities() {
        Ok(entries) => HttpResponse::Ok().json(entries),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}

pub fn search_cities(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
    query: web::Query<SearchQuery>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let dep = match &query.department {
        Some(dep) => dep,
        None => {
            return HttpResponse::BadRequest().body("Cannot search without 'department' parameter")
        }
    };

    let query = match &query.q {
        Some(query) => query,
        None => {
            return HttpResponse::BadRequest().body("Cannot search without query 'q' parameter")
        }
    };

    match domain.search_cities(dep.to_string(), query.to_string()) {
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

pub fn get_in_regional_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("region") {
        Some(region) => match domain.get_in_regional_index(region.to_string()) {
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

pub fn get_city_index(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("code_insee") {
        Some(code_insee) => match domain.get_city_index(code_insee.to_string()) {
            Ok(entry) => HttpResponse::Ok().json(entry),
            Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}

pub fn get_all_regional_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    _req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match domain.get_all_regions_index() {
        Ok(entry) => HttpResponse::Ok().json(entry),
        Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
    }
}

pub fn get_in_departmental_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
    query: web::Query<PageParam>
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("dept") {
        Some(dept) => match domain.get_in_departmental_index(dept.to_string(), query.page) {
                Ok(entry) => HttpResponse::Ok().json(entry),
                Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}

pub fn get_city_districts_index(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    match req.match_info().get("code_insee") {
        Some(code_insee) => match domain.get_city_districts_index(code_insee.to_string()) {
            Ok(entry) => HttpResponse::Ok().json(entry),
            Err(_) => HttpResponse::InternalServerError().body("Error with backend."),
        },
        None => HttpResponse::BadRequest().body("No region was given."),
    }
}
