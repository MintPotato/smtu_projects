use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotConfig {
    pub id: String,
    pub initial_position: (f32, f32, f32),
    pub video_source: String,
    pub network: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotState {
    pub position: (f32, f32, f32),
    pub temperature: f32,
    pub humidity: f32,
    pub timestamp: i64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    VideoFrame(Vec<u8>),
    RobotState(RobotState),
}
