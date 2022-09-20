use std::time::{Duration, Instant, SystemTime};

use serde::{Serialize, Deserialize};

const DEFAULT_DURATION: u64 = 15 * 60;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Option<String>,
    pub exp: u64,
    pub iat: u64,
}

impl Claims {
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let expiration = now + Duration::new(DEFAULT_DURATION, 0);
            
        Claims {
            aud: None,
            exp: expiration.as_secs(),
            iat: now.as_secs(),
        }
    }
}