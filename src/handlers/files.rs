use actix_web::{web, HttpRequest, Result};
use actix_files::NamedFile;
use tokio::fs;
use crate::config::{get_audio_dir, get_video_dir, get_merge_dir};
use crate::utils::is_safe_filename;

pub async fn serve_file(path: web::Path<String>, _req: HttpRequest) -> Result<NamedFile> {
    let filename = path.into_inner();
    
    if !is_safe_filename(&filename) {
        return Err(actix_web::error::ErrorBadRequest("Invalid filename"));
    }
    
    let dirs = vec![get_audio_dir(), get_video_dir(), get_merge_dir()];
    
    for dir in &dirs {
        let file_path = format!("{}/{}", dir, filename);
        
        if let Ok(canonical) = fs::canonicalize(&file_path).await {
            let dir_canonical = fs::canonicalize(dir).await.unwrap_or_default();
            
            if !canonical.starts_with(&dir_canonical) {
                return Err(actix_web::error::ErrorBadRequest("Invalid file path"));
            }
            
            if canonical.is_file() {
                let file = NamedFile::open(&file_path)?;
                
                return Ok(file
                    .use_last_modified(true)
                    .set_content_disposition(actix_web::http::header::ContentDisposition {
                        disposition: actix_web::http::header::DispositionType::Attachment,
                        parameters: vec![],
                    }));
            }
        }
    }
    
    Err(actix_web::error::ErrorNotFound("File not found"))
}
