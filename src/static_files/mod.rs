use crate::DEBUG_MODE;
use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("html/index.html"))
}

#[get("/application.css")]
pub fn application_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/application.css"))
}

#[get("/application.js")]
pub fn application_js() -> impl Responder {
    HttpResponse::Ok().body(include_str!("js/application.js"))
}

#[get("/bootstrap.css")]
pub fn bootstrap_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/bootstrap.min.css"))
}

#[get("/bootstrap.js")]
pub fn bootstrap_js() -> impl Responder {
    HttpResponse::Ok().body(include_str!("js/bootstrap.min.js"))
}

#[get("/vue.js")]
pub fn vue_js() -> impl Responder {
    if DEBUG_MODE {
        HttpResponse::Ok().body(include_str!("js/vue.debug.js"))
    } else {
        HttpResponse::Ok().body(include_str!("js/vue.min.js"))
    }
}
