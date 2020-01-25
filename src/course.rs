use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn icelandic() -> Course {
    serde_yaml::from_str(include_str!("../courses/icelandic.yaml")).unwrap()
}

#[get("/list")]
pub fn list_courses() -> impl Responder {
    let courses = [icelandic()];

    let courses_data = courses
        .iter()
        .map(|course| (course.id.as_str(), course.title.as_str()))
        .collect::<Vec<(&str, &str)>>();

    HttpResponse::Ok().json(courses_data)
}

#[get("/next")]
pub fn next_lesson() -> impl Responder {
    let course: Course = unimplemented!();
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
    let response = ChallengeResult::Accepted {
        explanation: "You made 1 mistake".to_string(),
    };

    HttpResponse::Ok().json(response)
}

/// A `Course` is a collection of lessons that teach a broader range of subjects around a central theme.
/// Eg. 'Single-variable calculus' or 'Icelandic language'.
#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub lessons: Vec<Lesson>,
}

/// A `Lesson` is a collection of challenges organised around a specific topic, eg. 'multiplying natural numbers'
/// or 'learning personal pronouns'.
#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub challenges: Vec<Challenge>,
}

/// A `Challenge` is a concrete question that is displayed to the user who is requested to answer it.
///
/// Challenges may have 'accepted' and 'allowed' answers to them. 'Accepted' answers are ones that are
/// perfectly correct and expected. In contrast to this, 'allowed' answers are technically correct but
/// they are either not following the question entirely or contain minor mistakes (eg. typos, synonyms, etc)
#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub question: String,
    pub accepted_answers: Vec<String>,
    pub allowed_answers: Vec<(String, String)>,
}

impl Challenge {
    /// Verifies the correctness of the given answer to this challenge
    pub fn verify(&self, response: &ChallengeResponse) -> ChallengeResult {
        ChallengeResult::Correct
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeResponse {
    course_id: String,
    challenge_id: String,
    answer: String,
}

#[derive(Serialize, Deserialize)]
pub enum ChallengeResult {
    /// The answer was perfectly correct and no correction was necessary
    Correct,
    /// The answer was accepted but had minor issues that had to be fixed. The mistakes are explained
    /// in the response
    Accepted { explanation: String },
    /// The answer contained major mistakes and therefore couldn't be accepted
    Incorrect,
}
