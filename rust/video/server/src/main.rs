use opencv; use core::str;
// dopolnit
use std::{
    fs, fmt,
    net::TcpStream,
};
use toml;
use serde::{de, Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct RobotConfig {
    data_source: String,
    pub robot: Robot,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Robot {
    humidity: f32,
    temperature: f32,
    position: Vec<f32>,
}


pub fn read_cfg() -> RobotConfig {
    let cfg = fs::read_to_string("./config.toml").unwrap_or_else(|_| {
        String::from(r#"
        data_source = "CAM:0"

        [drone]
        humidity = 69.0                 # f32
        temperature = 14.88              # f32
        robot_pos = [0.0, 0.0, 0.0]     # Vec<f32>
        "#)
    });

    let config: RobotConfig = toml::from_str(&cfg).expect("Failed to parse config");
    return config;
}

pub fn get_video(config: &RobotConfig) -> opencv::videoio::VideoCapture {
    
}