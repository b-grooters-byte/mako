use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

const DEFAULT_DURATION: u64 = 15 * 60;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub exp: u64,
    pub iat: u64,
}

impl Claims {
    pub fn new(audience: &str) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let expiration = now + Duration::new(DEFAULT_DURATION, 0);

        Claims {
            aud: audience.to_owned(),
            exp: expiration.as_secs(),
            iat: now.as_secs(),
        }
    }
}
