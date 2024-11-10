use axum::http::HeaderValue;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn generate_expires_header(days_in_future: u64) -> HeaderValue {
    let expiration = SystemTime::now() + Duration::from_secs(days_in_future * 86400); // days to seconds
    let unix_timestamp = expiration.duration_since(UNIX_EPOCH).unwrap().as_secs();
    HeaderValue::from_str(&format!("{}", unix_timestamp)).unwrap()
}
