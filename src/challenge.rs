use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use c2_chacha::stream_cipher::{NewStreamCipher, SyncStreamCipher};
use c2_chacha::ChaCha20;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[get("/next")]
pub async fn next_batch(session: Session) -> impl Responder {
    // Set "security token"
    let _ = session.set("security_token", [Utc::now().to_rfc2822()]);

    // Generate a batch
    let batch_size = 10;

    let mut response = vec![];

    for _ in 0..batch_size {
        let challenge = Challenge {
            task: "Translate this".to_string(),
            question: "How far can an European swallow fly?".to_string(),
            accepted: vec!["Not very far".to_string()],
        };

        let encrypted = challenge.encrypt();

        response.push(encrypted);
    }

    HttpResponse::Ok().json(response)
}

#[post("/verify")]
pub async fn verify_answer(
    _solution: web::Json<ChallengeSolution>,
    session: Session,
) -> impl Responder {
    let _r = session.get::<String>("");

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
    pub accepted: Vec<String>,
}

impl Challenge {
    /// Verifies the correctness of the given answer to this challenge
    pub fn _verify(&self, _solution: &ChallengeSolution) -> ChallengeResult {
        ChallengeResult {
            correct: true,
            explanation: None,
        }
    }

    pub fn encrypt(&self) -> Challenge {
        let mut encrypted = vec![];

        for accepted in &self.accepted {
            let normalized = accepted
                .to_lowercase()
                .trim()
                .chars()
                .filter(|e| e.is_alphanumeric())
                .collect::<String>();

            // Calculate hash for the normalised version
            let mut hasher = Sha256::new();
            hasher.input(normalized.as_str());

            let hash = hasher.result().to_vec();

            // Generate random 8-byte (u64) IV
            let iv = rand::thread_rng()
                .gen_range(0, u64::max_value())
                .to_ne_bytes();
            let mut iv_hex = base64::encode(&iv);

            // Use hash as encryption key
            let key = hash.as_slice();

            let mut cipher = ChaCha20::new_var(key, &iv).unwrap();
            let mut buffer = format!("{:04}:{}", accepted.as_bytes().len(), accepted);

            // Pad the buffer to a multiple of block size
            let padded_block_size = 64;
            let padded_length = ((buffer.bytes().len() + padded_block_size) / padded_block_size)
                * padded_block_size;
            let padding = format!("{:len$}", "", len = padded_length - buffer.len());

            buffer.push_str(padding.as_str());
            let mut data = buffer.bytes().collect::<Vec<u8>>();

            // Encrypt the expended buffer
            cipher.apply_keystream(&mut data);

            iv_hex.push_str(":");
            iv_hex.push_str(base64::encode(data.as_slice()).as_str());

            encrypted.push(iv_hex);
        }

        Challenge {
            task: self.task.to_string(),
            question: self.question.to_string(),
            accepted: encrypted,
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
    explanation: Option<String>,
}

#[cfg(test)]
mod tests {}
