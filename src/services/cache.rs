use actix_web::web;
use tokio::time::{sleep, Duration};
use chrono::Utc;
use crate::models::AppState;
use crate::config::get_cache_duration;

pub async fn cleanup_cache(state: web::Data<AppState>) {
    loop {
        sleep(Duration::from_secs(3600)).await;
        
        let now = Utc::now().timestamp_millis();
        let cache_duration = get_cache_duration();
        
        state.cache.retain(|_, entry| {
            if now - entry.timestamp > cache_duration {
                let _ = std::fs::remove_file(&entry.file_path);
                false
            } else {
                true
            }
        });
        
        state.pow_challenges.retain(|_, challenge| {
            now - challenge.timestamp < 300000
        });
        
        state.pow_sessions.retain(|_, session| {
            now - session.timestamp < 3600000
        });
    }
}
