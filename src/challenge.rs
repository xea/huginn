use actix_session::Session;
use actix_web::{post, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[post("/challenge/verify")]
pub async fn verify_answer(session: Session) -> impl Responder {
    let r = session.get::<String>("");

    let response = "";

    HttpResponse::Ok().json(response)
}

/// A `Challenge` is a concrete question that is displayed to the user who is requested to answer it.
///
/// Challenges may have 'accepted' and 'allowed' answers to them. 'Accepted' answers are ones that are
/// perfectly correct and expected. In contrast to this, 'allowed' answers are technically correct but
/// they are either not following the question entirely or contain minor mistakes (eg. typos, synonyms, etc)
#[derive(Serialize, Deserialize)]
pub struct Challenge {
    pub challenge_text: String,
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
    /// The answer was accepted but had minor issues were found
    Accepted,
    /// The answer contained major mistakes and therefore couldn't be accepted
    Incorrect,
}
