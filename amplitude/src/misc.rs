use std::time::{SystemTime, UNIX_EPOCH};

use afire::Response;
use rand::Rng;

use crate::error::StatusError;

pub enum LoginProvider {
    Google,
    Github,
}

pub fn current_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn rand_str(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(|x| x as char)
        .collect::<String>()
}

pub(crate) trait OkResponse {
    fn ok(self) -> Result<Response, StatusError>;
}

impl OkResponse for Response {
    fn ok(self) -> Result<Response, StatusError> {
        Ok(self)
    }
}
