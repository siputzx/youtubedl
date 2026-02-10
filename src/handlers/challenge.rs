use actix_web::{web, HttpRequest, HttpResponse, Result};
use chrono::Utc;
use crate::models::{ChallengeRequest, ChallengeResponse, PowChallenge, AppState};
use crate::config::get_pow_difficulty;
use crate::utils::{get_real_ip, get_client_identifier, generate_challenge};

pub async fn akumaudownload(
    body: web::Json<ChallengeRequest>,
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
    let challenge = generate_challenge();
    let difficulty = get_pow_difficulty();
    
    state.pow_challenges.insert(
        client_id.clone(),
        PowChallenge {
            challenge: challenge.clone(),
            url: body.url.clone(),
            media_type: body.r#type.clone(),
            difficulty,
            timestamp: Utc::now().timestamp_millis(),
        },
    );
    
    Ok(HttpResponse::Ok().json(ChallengeResponse {
        challenge,
        difficulty,
    }))
}
