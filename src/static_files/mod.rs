use crate::DEBUG_MODE;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("html/index.html"))
}

#[get("/application.css")]
pub async fn application_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/application.css"))
}

#[get("/application.js")]
pub async fn application_js() -> impl Responder {
    HttpResponse::Ok().body(include_str!("js/application.js"))
}

#[get("/bootstrap.css")]
pub async fn bootstrap_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/bootstrap.min.css"))
}

#[get("/bootstrap.js")]
pub async fn bootstrap_js() -> impl Responder {
    HttpResponse::Ok().body(include_str!("js/bootstrap.min.js"))
}

#[get("/nord.css")]
pub async fn nord_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/nord.css"))
}

#[get("/bulma.css")]
pub async fn bulma_css() -> impl Responder {
    HttpResponse::Ok().body(include_str!("css/bulma.min.css"))
}

#[get("/vue.js")]
pub async fn vue_js() -> impl Responder {
    if DEBUG_MODE {
        HttpResponse::Ok().body(include_str!("js/vue.debug.js"))
    } else {
        HttpResponse::Ok().body(include_str!("js/vue.min.js"))
    }
}

#[get("/svg/{image}")]
pub async fn images_svg(_image_name: web::Path<String>) -> impl Responder {
    //HttpResponse::Ok().body(include_str!("img/080-iceland.svg"))
    HttpResponse::Ok().body("SVG")
}
