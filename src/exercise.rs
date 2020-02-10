use actix_web::{get, Responder, HttpResponse, web};

#[get("/exercise/{course}/{skill}")]
pub async fn exercise_skill(skill_id: web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(include_str!("static_files/html/practice.html"))
}