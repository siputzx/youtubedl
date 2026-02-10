use actix_web::{web, HttpRequest, HttpResponse, Result, cookie::Cookie};
use chrono::Utc;
use crate::models::{VerifyRequest, PowSession, AppState};
use crate::services::youtube::extract_video_id;
use crate::utils::{get_real_ip, get_client_identifier, verify_pow, generate_session_token};

pub async fn cekpunyaku(
    body: web::Json<VerifyRequest>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let ip = get_real_ip(&req);
    
    let user_agent = req.headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    
    let client_id = get_client_identifier(&ip, &user_agent, &body.url, &body.r#type);
    
    let challenge = match state.pow_challenges.get(&client_id) {
        Some(c) => c.clone(),
        None => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Challenge not found or expired"
            })));
        }
    };
    
    if !verify_pow(&challenge.challenge, &body.nonce, challenge.difficulty) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid proof of work"
        })));
    }
    
    let video_id = match extract_video_id(&body.url) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid YouTube URL"})));
        }
    };
    
    let task_id = format!("{}_{}", video_id, body.r#type);
    let session_token = generate_session_token(&task_id, &ip, &user_agent);
    
    state.pow_sessions.insert(
        session_token.clone(),
        PowSession {
            task_id: task_id.clone(),
            ip: ip.clone(),
            user_agent: user_agent.clone(),
            timestamp: Utc::now().timestamp_millis(),
        },
    );
    
    state.pow_challenges.remove(&client_id);
    
    let cookie = Cookie::build("pow_session", session_token)
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(actix_web::cookie::SameSite::None)
        .finish();
    
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .finish())
}
