use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    // friends: Vec<String>,
    // name: String,
    pub status: Status,
    pub timestamps: Timestamps,
    // town: String,
    // uuid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub isNPC: bool,
    pub isOnline: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timestamps {
    pub lastOnline: i64
}