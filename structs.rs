#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

//player section
#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    // friends: Vec<String>,
    // name: String,
    pub stats: PlayerStats,
    pub status: PlayerStatus,
    pub timestamps: PlayerTimestamps
    // town: String,
    // uuid: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerStatus {
    pub isNPC: bool,
    pub isOnline: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerTimestamps {
    pub lastOnline: i64,
    pub registered: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerStats {
    pub balance: f32
}


//mainly town and a bit of nation section
#[derive(Debug, Serialize, Deserialize)]
pub struct Town {
    pub name: String,
    pub mayor: String,
    pub board: String,
    pub founder: String,
    pub nation: String,
    pub timestamps: TownTimestamps,
    pub coordinates: TownCoordinates,
    pub stats: TownStats,
    pub status: TownStatus
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownCoordinates {
    pub spawn: TownSpawn
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownSpawn {
    pub world: String,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownTimestamps {
    pub registered: i64,
    pub joinedNationAt: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownStats {
    pub balance: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TownStatus {
    pub isPublic: bool,
    pub isOpen: bool,
    pub isNeutral: bool,
    pub isCapital: bool,
    pub isOverClaimed: bool,
    pub isRuined: bool
}
