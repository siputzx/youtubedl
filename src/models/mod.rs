use serde::{Deserialize, Serialize};

pub mod state;
pub use state::AppState;

#[derive(Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub file_path: String,
    pub timestamp: i64,
    pub media_type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub url: String,
    pub media_type: String,
    pub status: String,
    pub progress: String,
    pub file_path: String,
    pub file_url: String,
    pub created_at: i64,
    pub error: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowChallenge {
    pub challenge: String,
    pub url: String,
    pub media_type: String,
    pub difficulty: usize,
    pub timestamp: i64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PowSession {
    pub task_id: String,
    pub ip: String,
    pub user_agent: String,
    pub timestamp: i64,
}

#[derive(Deserialize)]
pub struct DownloadQuery {
    pub url: String,
    pub r#type: String,
    pub apikey: Option<String>,
}

#[derive(Deserialize)]
pub struct ChallengeRequest {
    pub url: String,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub url: String,
    pub r#type: String,
    pub nonce: String,
}

#[derive(Serialize)]
pub struct ChallengeResponse {
    pub challenge: String,
    pub difficulty: usize,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
    pub system: SystemInfo,
    pub cache: CacheInfo,
    pub tasks: TaskInfo,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub uptime: String,
    pub version: String,
    pub platform: String,
    pub cpu_cores: usize,
    pub max_concurrent: usize,
    pub ffmpeg: String,
    pub ffprobe: String,
    pub active_cookies: String,
}

#[derive(Serialize)]
pub struct CacheInfo {
    pub total: usize,
}

#[derive(Serialize)]
pub struct TaskInfo {
    pub total: usize,
    pub downloading: usize,
    pub processing: usize,
    pub completed: usize,
    pub failed: usize,
}
