use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use chrono::Utc;

#[get("/next")]
pub async fn next_batch(session: Session) -> impl Responder {
    session.set("security_token", [
        Utc::now().to_rfc2822()
    ]);

    HttpResponse::Ok().json(())
}

#[post("/verify")]
pub async fn verify_answer(solution: web::Json<ChallengeSolution>, session: Session) -> impl Responder {
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
    pub task: String,
    pub question: String,
    pub accepted: Vec<String>
}

impl Challenge {
    /// Verifies the correctness of the given answer to this challenge
    pub fn verify(&self, solution: &ChallengeSolution) -> ChallengeResult {
        ChallengeResult {
            correct: true,
            explanation: None
        }
    }

    pub fn generate_stuff(&self) -> () {
        for accepted in self.accepted {
            let normalized = accepted;

        }
    }
}

/// A solution to a challenge as submitted by the user.
#[derive(Serialize, Deserialize)]
pub struct ChallengeSolution {
    user_input: String,
}

/// Represents the outcome of a solution along with some explanation if necessary.
///
/// Explanations might point out minor mistakes should they be present in the submitted answer.
#[derive(Serialize, Deserialize)]
pub struct ChallengeResult {
    correct: bool,
    explanation: Option<String>
}

#[cfg(test)]
mod tests {

}
