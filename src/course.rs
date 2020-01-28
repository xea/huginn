use crate::challenge::{ChallengeResponse, ChallengeResult};
use crate::lesson::Lesson;
use actix_web::{get, post, web, HttpResponse, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn icelandic() -> Course {
    serde_yaml::from_str(include_str!("../courses/icelandic.yaml")).unwrap()
}

pub fn all_courses() -> Vec<Course> {
    vec![icelandic()]
}

#[get("/list")]
pub fn list_courses() -> impl Responder {
    let courses = all_courses();
    let courses_data = courses
        .iter()
        .map(|course| &course.description)
        .collect::<Vec<&CourseDescription>>();

    HttpResponse::Ok().json(courses_data)
}

#[get("/show/{course_id}")]
pub fn show_course(course_id: web::Path<String>) -> impl Responder {
    let courses = all_courses();
    let course = courses
        .iter()
        .filter(|course| course.description.id == course_id.as_str())
        .next();

    HttpResponse::Ok().json(course)
}

// ----------- Here be dragons

#[get("/next")]
pub fn next_lesson() -> impl Responder {
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

#[post("/submit")]
pub fn submit_answer(_response: web::Json<ChallengeResponse>) -> impl Responder {
    println!("Got request yay");
    let response = ChallengeResult::Accepted {
        explanation: "You made 1 mistake".to_string(),
    };

    HttpResponse::Ok().json(response)
}

/// A `Course` is a collection of lessons that teach a broader range of subjects around a central theme.
/// Eg. 'Single-variable calculus' or 'Icelandic language'.
#[derive(Serialize, Deserialize)]
pub struct Course {
    pub description: CourseDescription,
    pub lessons: Vec<Lesson>,
}

#[derive(Serialize, Deserialize)]
pub struct CourseDescription {
    pub id: String,
    pub title: String,
}
