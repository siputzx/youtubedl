use actix_web::web;
use tokio::fs;
use tokio::time::{sleep, Duration};
use uuid::Uuid;
use chrono::Utc;
use std::path::PathBuf;
use dashmap::DashMap;
use crate::models::{Task, CacheEntry, AppState};
use crate::config::{get_audio_dir, get_video_dir, get_merge_dir, get_max_video_duration, get_max_audio_duration, get_max_file_size};
use crate::services::youtube::{get_video_info, get_format_hierarchy, execute_ytdlp};
use crate::utils::helpers::get_random_cookies;

pub async fn find_file(dir: &str, uuid: &str, ext: &str) -> Option<String> {
    let expected_path = format!("{}/{}{}", dir, uuid, ext);
    
    match fs::metadata(&expected_path).await {
        Ok(metadata) if metadata.is_file() => Some(expected_path),
        _ => None,
    }
}

pub fn update_task_status<F>(tasks: &DashMap<String, Task>, task_id: &str, updater: F)
where
    F: FnOnce(&mut Task),
{
    if let Some(mut task) = tasks.get_mut(task_id) {
        updater(&mut task);
    }
}

pub async fn process_download(state: web::Data<AppState>, video_id: String, url: String, media_type: String) {
    let task_id = format!("{}_{}", video_id, media_type);
    
    let dir = match media_type.as_str() {
        "audio" => get_audio_dir(),
        "video" => get_video_dir(),
        "merge" => get_merge_dir(),
        _ => {
            update_task_status(&state.tasks, &task_id, |task| {
                task.status = "failed".to_string();
                task.error = "Invalid media type processing".to_string();
            });
            let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, media_type)));
            state.cache.remove(&cache_key);
            sleep(Duration::from_secs(3)).await;
            state.tasks.remove(&task_id);
            return;
        }
    };

    let cookies = get_random_cookies();
    let cookies_ref = cookies.as_deref();

    if let Some((duration, filesize)) = get_video_info(&url, cookies_ref).await {
        let max_duration = if media_type == "audio" { get_max_audio_duration() } else { get_max_video_duration() };
        
        if duration > max_duration as f64 {
            update_task_status(&state.tasks, &task_id, |task| {
                task.status = "failed".to_string();
                task.error = "Duration exceeds maximum".to_string();
            });
            let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, media_type)));
            state.cache.remove(&cache_key);
            sleep(Duration::from_secs(3)).await;
            state.tasks.remove(&task_id);
            return;
        }
        
        let max_file_size = get_max_file_size();
        if filesize > 0 && filesize > max_file_size {
            update_task_status(&state.tasks, &task_id, |task| {
                task.status = "failed".to_string();
                task.error = "File size exceeds maximum".to_string();
            });
            let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, media_type)));
            state.cache.remove(&cache_key);
            sleep(Duration::from_secs(3)).await;
            state.tasks.remove(&task_id);
            return;
        }
    }

    let filename = Uuid::new_v4().to_string();
    let format_hierarchy = get_format_hierarchy(&media_type);
    
    let mut success = false;
    let mut final_file_path = String::new();

    update_task_status(&state.tasks, &task_id, |task| {
        task.status = "downloading".to_string();
        task.progress = "50%".to_string();
    });

    for (format, ext, post_proc) in format_hierarchy {
        let output_template = format!("{}/{}.%(ext)s", dir, filename);

        match execute_ytdlp(&url, format, &output_template, post_proc, state.max_concurrent, &state.ffmpeg_path, cookies_ref).await {
            Ok(_) => {
                update_task_status(&state.tasks, &task_id, |task| {
                    task.status = "processing".to_string();
                    task.progress = "100%".to_string();
                });

                if let Some(file_path) = find_file(&dir, &filename, ext).await {
                    final_file_path = file_path;
                    success = true;
                    break;
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        }
    }

    if success {
        let file_url = format!("/files/{}", PathBuf::from(&final_file_path).file_name().unwrap().to_string_lossy());
        
        let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, media_type)));
        state.cache.insert(cache_key, CacheEntry {
            file_path: final_file_path.clone(),
            timestamp: Utc::now().timestamp_millis(),
            media_type: media_type.clone(),
        });

        update_task_status(&state.tasks, &task_id, |task| {
            task.status = "completed".to_string();
            task.progress = "100%".to_string();
            task.file_path = final_file_path.clone();
            task.file_url = file_url;
        });
    } else {
        update_task_status(&state.tasks, &task_id, |task| {
            task.status = "failed".to_string();
            task.error = "All format attempts failed".to_string();
        });
        let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, media_type)));
        state.cache.remove(&cache_key);
        sleep(Duration::from_secs(3)).await;
        state.tasks.remove(&task_id);
    }
}
