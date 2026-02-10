use dashmap::DashMap;
use std::time::Instant;
use crate::models::{CacheEntry, Task, PowChallenge, PowSession};

pub struct AppState {
    pub cache: DashMap<String, CacheEntry>,
    pub tasks: DashMap<String, Task>,
    pub pow_challenges: DashMap<String, PowChallenge>,
    pub pow_sessions: DashMap<String, PowSession>,
    pub start_time: Instant,
    pub max_concurrent: usize,
    pub ffmpeg_path: String,
    pub ffprobe_path: String,
}

impl AppState {
    pub fn new(max_concurrent: usize, ffmpeg_path: String, ffprobe_path: String) -> Self {
        Self {
            cache: DashMap::new(),
            tasks: DashMap::new(),
            pow_challenges: DashMap::new(),
            pow_sessions: DashMap::new(),
            start_time: Instant::now(),
            max_concurrent,
            ffmpeg_path,
            ffprobe_path,
        }
    }
}
