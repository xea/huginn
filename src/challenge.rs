use actix_session::Session;
use actix_web::{get, HttpResponse, Responder};
use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::BlockMode;
use block_modes::Cbc;
use chrono::Utc;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const SECURITY_TOKEN: &str = "security_token";

#[get("/next")]
pub async fn next_batch(session: Session) -> impl Responder {
    // Set "security token"
    let _ = session.set(SECURITY_TOKEN, [Utc::now().to_rfc2822()]);

    // Generate a batch
    let batch_size = 10;

    let mut response = vec![];

    for _ in 0..batch_size {
        let challenge: Challenge = unimplemented!();

        let encrypted = challenge.encrypt();

        response.push(encrypted);
    }

    HttpResponse::Ok().json(response)
}

/// A `Challenge` is a concrete question that is displayed to the user who is requested to answer it.
///
#[derive(Serialize, Deserialize)]
pub struct Challenge {
    /// Gives some instruction to the user about what they need to do in the current challenge.
    /// eg. 'Solve the following equation'
    pub task: String,

    /// The textual representation of the actual challenge.
    /// eg. 'What is the square root of 3?'
    pub question: String,

    /// All the accepted answers
    pub accepted: Vec<String>,
}

impl Challenge {
    /// Generate a copy of this challenge but with the accepted answers encrypted.
    pub fn encrypt(&self) -> Challenge {
        let mut encrypted = vec![];

        for accepted in &self.accepted {
            let normalized = Challenge::normalize(accepted.as_str());

            let encrypted_answer = Challenge::encrypt_raw(normalized);

            encrypted.push(encrypted_answer);
        }

        Challenge {
            task: self.task.to_string(),
            question: self.question.to_string(),
            accepted: encrypted,
        }
    }

    fn normalize(input: &str) -> String {
        input
            .to_lowercase()
            .trim()
            .chars()
            .filter(|e| e.is_alphanumeric())
            .collect::<String>()
    }

    fn encrypt_raw(input: String) -> String {
        Self::encrypt_aes(input)
    }

    /// Take an input and generate an encrypted version of the string that can be sent to the client
    /// for verifying the correctness of their input.
    ///
    /// Specifically, the user input will be normalised (that is, all whitespaces and non-alphanumeric
    /// characters removed and the result lower-cased) first, and a SHA256 hash is calculated from it.
    ///
    /// The normalised input, along with it's length will be padded to the nearest 32 bytes and the
    /// whole string then AES128 encrypted with the first 16 bytes of the hash as an encryption key.
    ///
    /// Despite the name, this method does not want and does not need to be cryptographically secure.
    /// Cryptographic primitives are only used to provide some degree of resistance against reverse-
    /// engineering the challenge data.
    ///
    /// It only utilises AES-CBC and SHA256 because they are generally available algorithms that are
    /// provided on most platforms out of the box.
    fn encrypt_aes(input: String) -> String {
        // Calculate hash for the normalised version
        let mut hasher = Sha256::new();
        hasher.input(input.as_str());

        // We only need the first 16 bytes of the 32 byte long Sha256 hash
        let hash = hasher.result()[0..16].to_vec();

        // Generate random 16-byte (u128) IV
        let iv = thread_rng().gen_range(0, u128::max_value()).to_ne_bytes();
        // The first part of the output is the base64-encoded IV
        let mut output = base64::encode(&iv);

        // Use hash as encryption key
        let key = hash.as_slice();

        let mut buffer = format!("{:04}:{}", input.as_bytes().len(), input);

        // Pad the buffer to a multiple of block size
        let padded_block_size = 32;
        let padded_length =
            ((buffer.bytes().len() + padded_block_size) / padded_block_size) * padded_block_size;
        let padding = format!("{:len$}", "", len = padded_length - buffer.len());

        buffer.push_str(padding.as_str());
        // Data is the final version of our input to be encrypted. It is in the format of <length>:<content><padding>
        let data = buffer.bytes().collect::<Vec<u8>>();

        let cipher = Cbc::<Aes128, Pkcs7>::new_var(key, &iv).unwrap();
        let cipher_text = cipher.encrypt_vec(data.as_slice());

        output.push(':');
        output.push_str(base64::encode(cipher_text.as_slice()).as_str());

        output
    }
}

#[cfg(test)]
mod tests {}
