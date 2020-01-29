use crate::challenge::Challenge;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::course::all_courses;

#[get("/list")]
pub async fn list_lessons() -> impl Responder {
    let courses = all_courses();
    let lessons = courses .iter()
        .flat_map(|course| &course.lessons)
        .map(|lesson| &lesson.description)
        .collect::<Vec<&LessonDescription>>();

    HttpResponse::Ok().json(lessons)
}

#[get("/show/{lesson_id}")]
pub async fn show_lesson(lesson_id: web::Path<String>) -> impl Responder {
    let courses = all_courses();

    HttpResponse::Ok().body("asdfasdf")
}

/// A `Lesson` is a collection of challenges organised around a specific topic, eg. 'multiplying natural numbers'
/// or 'learning personal pronouns'.
#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub description: LessonDescription,
    pub challenges: Vec<Challenge>,
}

#[derive(Serialize, Deserialize)]
pub struct LessonDescription {
    pub id: String,
    pub title: String,
}
