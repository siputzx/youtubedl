use chrono::Local;

pub fn log_request(method: &str, path: &str, status: u16, latency_ms: u128) {
    println!(
        "[{}] {} {} - {} ({}ms)",
        Local::now().format("%H:%M:%S"),
        method,
        path,
        status,
        latency_ms
    );
}

#[allow(dead_code)]
pub fn log_error(msg: &str) {
    println!("[{}] ERROR: {}", Local::now().format("%H:%M:%S"), msg);
}

pub fn log_startup(msg: &str) {
    println!("{}", msg);
}
