use serde::{Deserialize, Serialize};

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
    pub fn verify(&self, _response: &ChallengeResponse) -> ChallengeResult {
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
