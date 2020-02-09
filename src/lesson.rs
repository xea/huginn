use crate::challenge::Challenge;
use crate::course::all_courses;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[get("/list")]
pub async fn list_lessons() -> impl Responder {
    let courses = all_courses();
    let lessons = courses
        .iter()
        .flat_map(|course| &course.lessons)
        .collect::<Vec<&Lesson>>();

    HttpResponse::Ok().json(lessons)
}

#[get("/show/{lesson_id}")]
pub async fn show_lesson(_lesson_id: web::Path<String>) -> impl Responder {
    let _courses = all_courses();

    HttpResponse::Ok().body("asdfasdf")
}

/// A `Lesson` is a collection of challenges organised around a specific topic, eg. 'multiplying natural numbers'
/// or 'learning personal pronouns'.
#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    #[serde(skip)]
    pub challenges: Vec<Challenge>,
}
