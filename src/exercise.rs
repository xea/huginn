use actix_web::{get, Responder, HttpResponse};

#[get("/exercise/{course}/{skill}")]
pub async fn exercise_skill() -> impl Responder {
    HttpResponse::Ok().body(include_str!("static_files/html/practice.html"))
}