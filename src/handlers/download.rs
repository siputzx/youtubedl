use actix_web::{web, HttpRequest, HttpResponse, Result};
use chrono::Utc;
use std::path::PathBuf;
use crate::models::{DownloadQuery, Task, AppState};
use crate::services::{extract_video_id, process_download};
use crate::middleware::is_valid_apikey;
use crate::utils::get_real_ip;

pub async fn download(query: web::Query<DownloadQuery>, state: web::Data<AppState>, req: HttpRequest) -> Result<HttpResponse> {
    match query.r#type.as_str() {
        "audio" | "video" | "merge" => (),
        _ => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid media type. Allowed: audio, video, merge",
                "received": query.r#type
            })));
        }
    }

    let is_premium = query.apikey.as_ref().map_or(false, |key| is_valid_apikey(key));

    let video_id = match extract_video_id(&query.url) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid YouTube URL"})));
        }
    };

    let cache_key = format!("{:x}", md5::compute(format!("{}_{}", video_id, query.r#type)));
    
    if let Some(entry) = state.cache.get(&cache_key) {
        if PathBuf::from(&entry.file_path).exists() {
            if !is_premium {
                let session_cookie = req.cookie("pow_session");
                if session_cookie.is_none() {
                    return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "PoW challenge required",
                        "action": "get_challenge"
                    })));
                }
            }
            
            let file_url = format!("/files/{}", PathBuf::from(&entry.file_path).file_name().unwrap().to_string_lossy());
            return Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "completed",
                "fileUrl": file_url,
                "cached": true
            })));
        } else {
            state.cache.remove(&cache_key);
        }
    }

    if !is_premium {
        let session_cookie = req.cookie("pow_session");
        
        if let Some(cookie) = session_cookie {
            let session_token = cookie.value().to_string();
            
            let ip = get_real_ip(&req);
            
            let user_agent = req.headers()
                .get("user-agent")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown")
                .to_string();
            
            let task_id = format!("{}_{}", video_id, query.r#type);
            
            match state.pow_sessions.get(&session_token) {
                Some(session) => {
                    if session.task_id != task_id || session.ip != ip || session.user_agent != user_agent {
                        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
                            "error": "Invalid session for this task"
                        })));
                    }
                }
                None => {
                    return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                        "error": "PoW challenge required",
                        "action": "get_challenge"
                    })));
                }
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "PoW challenge required",
                "action": "get_challenge"
            })));
        }
    }

    let task_id = format!("{}_{}", video_id, query.r#type);
    
    if let Some(task) = state.tasks.get(&task_id) {
        return Ok(HttpResponse::Ok().json(task.clone()));
    }

    let task = Task {
        id: task_id.clone(),
        url: query.url.clone(),
        media_type: query.r#type.clone(),
        status: "downloading".to_string(),
        progress: "50%".to_string(),
        file_path: String::new(),
        file_url: String::new(),
        created_at: Utc::now().timestamp_millis(),
        error: String::new(),
    };

    state.tasks.insert(task_id.clone(), task.clone());

    let state_clone = state.clone();
    tokio::spawn(async move {
        process_download(state_clone, video_id, query.url.clone(), query.r#type.clone()).await;
    });

    Ok(HttpResponse::Accepted().json(task))
}
