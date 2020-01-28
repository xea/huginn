use crate::challenge::Challenge;
use actix_web::{get, web, Responder};
use serde::{Deserialize, Serialize};

#[get("/list")]
pub fn list_lessons() -> impl Responder {
    unimplemented!()
}

#[get("/show/{lesson_id}")]
pub fn show_lesson(lesson_id: web::Path<String>) -> impl Responder {
    unimplemented!()
}

/// A `Lesson` is a collection of challenges organised around a specific topic, eg. 'multiplying natural numbers'
/// or 'learning personal pronouns'.
#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub challenges: Vec<Challenge>,
}
