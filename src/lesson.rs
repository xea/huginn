use crate::challenge::Challenge;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/list")]
pub async fn list_lessons() -> impl Responder {
    HttpResponse::Ok().body("asdfasdf")
}

#[get("/show/{lesson_id}")]
pub async fn show_lesson(lesson_id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body("asdfasdf")
}

/// A `Lesson` is a collection of challenges organised around a specific topic, eg. 'multiplying natural numbers'
/// or 'learning personal pronouns'.
#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub challenges: Vec<Challenge>,
}
