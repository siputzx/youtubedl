use actix_web::{web, App, HttpServer};

mod config;
mod models;
mod utils;
mod services;
mod middleware;
mod handlers;

use config::{get_downloads_dir, get_audio_dir, get_video_dir, get_merge_dir, get_cookies_dir, get_port, get_max_concurrent};
use models::AppState;
use utils::{log_startup, find_executable};
use services::cleanup_cache;
use handlers::{akumaudownload, cekpunyaku, status, download, serve_file};
use middleware::RequestLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let downloads_dir = get_downloads_dir();
    let audio_dir = get_audio_dir();
    let video_dir = get_video_dir();
    let merge_dir = get_merge_dir();
    let cookies_dir = get_cookies_dir();
    
    for dir in &[&downloads_dir, &audio_dir, &video_dir, &merge_dir, &cookies_dir] {
        tokio::fs::create_dir_all(dir).await?;
    }

    let cpu_cores = num_cpus::get();
    let max_concurrent = get_max_concurrent().unwrap_or_else(|| (cpu_cores * 2).max(4).min(32));

    let ffmpeg_path = find_executable("ffmpeg");
    let ffprobe_path = find_executable("ffprobe");

    let state = web::Data::new(AppState::new(max_concurrent, ffmpeg_path, ffprobe_path));

    let state_clone = state.clone();
    tokio::spawn(async move {
        cleanup_cache(state_clone).await;
    });

    let port = get_port();
    let bind_addr = format!("0.0.0.0:{}", port);
    
    log_startup(&format!("YouTube Downloader v2.0.0 - Listening on {}", bind_addr));

    HttpServer::new(move || {
        App::new()
            .wrap(RequestLogger)
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
            )
            .app_data(state.clone())
            .route("/", web::get().to(status))
            .route("/akumaudownload", web::post().to(akumaudownload))
            .route("/cekpunyaku", web::post().to(cekpunyaku))
            .route("/download", web::get().to(download))
            .route("/files/{filename}", web::get().to(serve_file))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
