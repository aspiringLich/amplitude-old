use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawTrackConfig {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub description: String,
}

impl Track {
    pub fn from_raw(raw: &RawTrackConfig, id: String) -> Self {
        Self {
            id,
            title: raw.title.clone(),
            description: raw.description.clone(),
        }
    }
}
