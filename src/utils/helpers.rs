use actix_web::HttpRequest;
use std::path::PathBuf;
use std::fs;
use rand::Rng;
use crate::config::{get_cookies_dir, use_cookies};

pub fn get_real_ip(req: &HttpRequest) -> String {
    req.headers()
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
        })
        .or_else(|| {
            req.headers()
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.split(',').next())
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| {
            req.peer_addr()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        })
}

pub fn get_available_cookies() -> Vec<String> {
    if !use_cookies() {
        return Vec::new();
    }
    
    let cookies_dir = get_cookies_dir();
    let cookies_path = PathBuf::from(&cookies_dir);
    
    if !cookies_path.exists() || !cookies_path.is_dir() {
        return Vec::new();
    }
    
    let mut cookie_files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&cookies_path) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if let Some(filename) = entry.file_name().to_str() {
                        if filename.starts_with("cookies") && filename.ends_with(".txt") {
                            if let Ok(path) = entry.path().canonicalize() {
                                cookie_files.push(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }
    
    cookie_files.sort();
    cookie_files
}

pub fn get_random_cookies() -> Option<String> {
    let available = get_available_cookies();
    
    if available.is_empty() {
        return None;
    }
    
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..available.len());
    Some(available[index].clone())
}

pub fn is_safe_filename(filename: &str) -> bool {
    !filename.contains("..") && !filename.contains('/') && !filename.contains('\\')
}

pub fn find_executable(name: &str) -> String {
    for path in &[
        format!("/usr/bin/{}", name),
        format!("/usr/local/bin/{}", name),
        format!("/bin/{}", name),
        format!("/opt/bin/{}", name),
    ] {
        if PathBuf::from(path).exists() {
            return path.clone();
        }
    }
    which::which(name).ok().and_then(|p| p.to_str().map(String::from)).unwrap_or_default()
}
