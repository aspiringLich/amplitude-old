use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

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
