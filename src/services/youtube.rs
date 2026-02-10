use regex::Regex;
use tokio::process::Command;
use crate::config::get_proxy;

pub fn extract_video_id(url: &str) -> Result<String, String> {
    let re = Regex::new(r"(?:youtube\.com\/(?:watch\?v=|shorts\/|embed\/|v\/)|youtu\.be\/|music\.youtube\.com\/watch\?v=|googleusercontent\.com\/youtube\.com\/[0-2])([a-zA-Z0-9_-]{10,12})").unwrap();

    if let Some(caps) = re.captures(url) {
        Ok(caps[1].to_string())
    } else if url.len() == 11 && !url.contains('/') && !url.contains('.') {
        Ok(url.to_string())
    } else {
        Err("Invalid YouTube URL or ID".to_string())
    }
}

pub async fn get_video_info(url: &str, cookies: Option<&str>) -> Option<(f64, u64)> {
    let mut args = vec![
        "--dump-json".to_string(),
        "--no-playlist".to_string(),
        "--remote-components".to_string(),
        "ejs:github".to_string(),
        url.to_string(),
    ];

    if let Some(cookie_file) = cookies {
        args.insert(0, "--cookies".to_string());
        args.insert(1, cookie_file.to_string());
    }

    if let Some(proxy) = get_proxy() {
        args.insert(0, "--proxy".to_string());
        args.insert(1, proxy);
    }

    let output = Command::new("yt-dlp").args(&args).output().await.ok()?;
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    
    let duration = json["duration"].as_f64()?;
    let filesize = json["filesize"].as_u64()
        .or_else(|| json["filesize_approx"].as_u64())
        .unwrap_or(0);

    Some((duration, filesize))
}

pub fn get_format_hierarchy(media_type: &str) -> Vec<(&'static str, &'static str, &'static str)> {
    match media_type {
        "audio" => vec![
            ("bestaudio[abr<=128]", ".mp3", "audio"),
            ("bestaudio[abr<=192]", ".mp3", "audio"),
            ("bestaudio", ".mp3", "audio"),
        ],
        "video" => vec![
            ("bestvideo[height=720][fps=60]", ".mp4", "video"),
            ("bestvideo[height=720]", ".mp4", "video"),
            ("bestvideo[height=480]", ".mp4", "video"),
            ("bestvideo[height=360]", ".mp4", "video"),
            ("bestvideo[height=1080]", ".mp4", "video"),
            ("bestvideo[height=1440]", ".mp4", "video"),
            ("bestvideo[height=2160]", ".mp4", "video"),
            ("bestvideo", ".mp4", "video"),
        ],
        "merge" => vec![
            ("bestvideo[height=720][fps=60]+bestaudio[abr<=192]/best[height=720][fps=60]", ".mp4", "video"),
            ("bestvideo[height=720]+bestaudio[abr<=192]/best[height=720]", ".mp4", "video"),
            ("bestvideo[height=480]+bestaudio[abr<=128]/best[height=480]", ".mp4", "video"),
            ("bestvideo[height=360]+bestaudio/best[height=360]", ".mp4", "video"),
            ("bestvideo[height=1080]+bestaudio/best[height=1080]", ".mp4", "video"),
            ("bestvideo[height=1440]+bestaudio/best[height=1440]", ".mp4", "video"),
            ("bestvideo[height=2160]+bestaudio/best[height=2160]", ".mp4", "video"),
            ("bestvideo+bestaudio/best", ".mp4", "video"),
        ],
        _ => vec![],
    }
}

pub async fn execute_ytdlp(
    url: &str,
    format: &str,
    output: &str,
    post_proc: &str,
    max_concurrent: usize,
    ffmpeg: &str,
    cookies: Option<&str>,
) -> Result<(), String> {
    let mut args = vec![
        "-f".to_string(), format.to_string(),
        "-o".to_string(), output.to_string(),
        "--no-playlist".to_string(),
        "--no-warnings".to_string(),
        "--remote-components".to_string(), "ejs:github".to_string(),
        "--concurrent-fragments".to_string(), max_concurrent.to_string(),
        "--buffer-size".to_string(), "1M".to_string(),
        "--http-chunk-size".to_string(), "10M".to_string(),
        "--throttled-rate".to_string(), "100K".to_string(),
        "--retries".to_string(), "10".to_string(),
        "--fragment-retries".to_string(), "10".to_string(),
        "--file-access-retries".to_string(), "10".to_string(),
        "--no-part".to_string(),
        "--no-mtime".to_string(),
        "--continue".to_string(),
    ];

    if let Some(cookie_file) = cookies {
        args.push("--cookies".to_string());
        args.push(cookie_file.to_string());
    }

    if let Some(proxy) = get_proxy() {
        args.push("--proxy".to_string());
        args.push(proxy);
    }

    if !ffmpeg.is_empty() {
        args.push("--ffmpeg-location".to_string());
        args.push(ffmpeg.to_string());
    }

    if post_proc == "audio" {
        args.extend(vec![
            "-x".to_string(),
            "--audio-format".to_string(), "mp3".to_string(),
            "--audio-quality".to_string(), "0".to_string(),
            "--embed-metadata".to_string(),
            "--embed-thumbnail".to_string(),
            "--convert-thumbnails".to_string(), "jpg".to_string(),
        ]);
    } else if post_proc == "video" {
        args.extend(vec![
            "--merge-output-format".to_string(), "mp4".to_string(),
            "--remux-video".to_string(), "mp4".to_string(),
        ]);
    }

    args.push(url.to_string());

    let output = Command::new("yt-dlp")
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("yt-dlp error: {}", e))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(format!("yt-dlp failed: {}", error_msg));
    }

    Ok(())
}
