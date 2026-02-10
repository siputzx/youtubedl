use std::env;

pub fn get_downloads_dir() -> String {
    env::var("DOWNLOADS_DIR").unwrap_or_else(|_| "./downloads".to_string())
}

pub fn get_audio_dir() -> String {
    env::var("AUDIO_DIR").unwrap_or_else(|_| "./downloads/audio".to_string())
}

pub fn get_video_dir() -> String {
    env::var("VIDEO_DIR").unwrap_or_else(|_| "./downloads/video".to_string())
}

pub fn get_merge_dir() -> String {
    env::var("MERGE_DIR").unwrap_or_else(|_| "./downloads/merge".to_string())
}

pub fn get_cookies_dir() -> String {
    env::var("COOKIES_DIR").unwrap_or_else(|_| "./cookies".to_string())
}

pub fn get_cache_duration() -> i64 {
    env::var("CACHE_DURATION")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(18000000)
}

pub fn get_max_video_duration() -> i64 {
    env::var("MAX_VIDEO_DURATION")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10800)
}

pub fn get_max_audio_duration() -> i64 {
    env::var("MAX_AUDIO_DURATION")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(18000)
}

pub fn get_max_file_size() -> u64 {
    env::var("MAX_FILE_SIZE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1073741824)
}

pub fn get_pow_difficulty() -> usize {
    env::var("POW_DIFFICULTY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

pub fn get_valid_apikeys() -> Vec<String> {
    env::var("VALID_APIKEYS")
        .map(|keys| keys.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|_| vec![
            "nbteam".to_string(),
            "siputzxteam".to_string(),
            "cepetan".to_string(),
        ])
}

pub fn get_port() -> String {
    env::var("PORT").unwrap_or_else(|_| "3004".to_string())
}

pub fn get_max_concurrent() -> Option<usize> {
    env::var("MAX_CONCURRENT")
        .ok()
        .and_then(|v| v.parse().ok())
}

pub fn use_cookies() -> bool {
    env::var("USE_COOKIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(true)
}

pub fn get_proxy() -> Option<String> {
    env::var("PROXY").ok().filter(|s| !s.is_empty())
}
