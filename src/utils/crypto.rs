use sha2::{Sha256, Digest};
use uuid::Uuid;
use chrono::Utc;

pub fn get_client_identifier(ip: &str, user_agent: &str, url: &str, media_type: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(user_agent.as_bytes());
    hasher.update(url.as_bytes());
    hasher.update(media_type.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn generate_challenge() -> String {
    let mut hasher = Sha256::new();
    hasher.update(Uuid::new_v4().to_string().as_bytes());
    hasher.update(Utc::now().timestamp_millis().to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn verify_pow(challenge: &str, nonce: &str, difficulty: usize) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(challenge.as_bytes());
    hasher.update(nonce.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    
    let prefix = "0".repeat(difficulty);
    hash.starts_with(&prefix)
}

pub fn generate_session_token(task_id: &str, ip: &str, user_agent: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(task_id.as_bytes());
    hasher.update(ip.as_bytes());
    hasher.update(user_agent.as_bytes());
    hasher.update(Utc::now().timestamp_millis().to_string().as_bytes());
    format!("{:x}", hasher.finalize())
}
