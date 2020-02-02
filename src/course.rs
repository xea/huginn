use crate::lesson::Lesson;
use actix_web::{get, web, HttpResponse, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn icelandic() -> Course {
    Course {
        id: "icelandic".to_string(),
        title: "Icelandic Language".to_string(),
        lessons: vec![],
    }
}

pub fn all_courses() -> Vec<Course> {
    vec![icelandic()]
}

#[get("/list")]
pub async fn list_courses() -> impl Responder {
    let courses = all_courses();

    HttpResponse::Ok().json(courses)
}

#[get("/show/{course_id}")]
pub async fn show_course(course_id: web::Path<String>) -> impl Responder {
    let courses = all_courses();
    let course = courses
        .iter()
        .filter(|course| course.id == course_id.as_str())
        .next();

    HttpResponse::Ok().json(course)
}

// ----------- Here be dragons

#[get("/next")]
pub async fn next_lesson() -> impl Responder {
    let course = icelandic();
    let lesson_idx = rand::thread_rng().gen_range(0, course.lessons.len());

    HttpResponse::Ok().json(
        course
            .lessons
            .get(lesson_idx)
            .filter(|lesson| !lesson.challenges.is_empty())
            .map(|lesson| &lesson.challenges)
            .and_then(|challenges| {
                let challenge_idx = rand::thread_rng().gen_range(0, challenges.len());

                challenges.get(challenge_idx)
            }),
    )
}

/// A `Course` is a collection of lessons that teach a broader range of subjects around a central theme.
/// Eg. 'Single-variable calculus' or 'Icelandic language'.
#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,

    #[serde(skip)]
    pub lessons: Vec<Lesson>,
}
