use actix_web::{web, HttpResponse, Result};
use crate::models::{StatusResponse, SystemInfo, CacheInfo, TaskInfo, AppState};
use crate::utils::helpers::get_available_cookies;

pub async fn status(state: web::Data<AppState>) -> Result<HttpResponse> {
    let uptime = state.start_time.elapsed();
    let total = state.tasks.len();

    let (downloading, processing, completed, failed) = state.tasks.iter()
        .fold((0, 0, 0, 0), |(d, p, c, f), entry| {
            match entry.status.as_str() {
                "downloading" => (d + 1, p, c, f),
                "processing" => (d, p + 1, c, f),
                "completed" => (d, p, c + 1, f),
                "failed" => (d, p, c, f + 1),
                _ => (d, p, c, f),
            }
        });

    let cookies_count = get_available_cookies().len();
    let active_cookies = if cookies_count > 0 {
        format!("{} files", cookies_count)
    } else {
        "none".to_string()
    };

    Ok(HttpResponse::Ok().json(StatusResponse {
        status: "ok".to_string(),
        system: SystemInfo {
            uptime: format!("{:?}", uptime),
            version: "2.0.0".to_string(),
            platform: "youtube-downloader".to_string(),
            cpu_cores: num_cpus::get(),
            max_concurrent: state.max_concurrent,
            ffmpeg: state.ffmpeg_path.clone(),
            ffprobe: state.ffprobe_path.clone(),
            active_cookies,
        },
        cache: CacheInfo {
            total: state.cache.len(),
        },
        tasks: TaskInfo {
            total,
            downloading,
            processing,
            completed,
            failed,
        },
    }))
}
