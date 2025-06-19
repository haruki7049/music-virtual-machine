use serde::{Serialize, Deserialize};
use std::path::Path;

pub fn parse<P: AsRef<Path>>(path: P) -> Result<MusicData, Box<dyn std::error::Error>> {
    let data: String = std::fs::read_to_string(path)?;
    let music_data: Result<MusicData, toml::de::Error> = toml::from_str(&data);
    match music_data {
        Ok(data) => return Ok(data),
        Err(err) => panic!("{}", err),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicData {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub sample_format: SampleFormat,

    pub bits: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SampleFormat {
    Float,
    Int,
}

impl From<self::SampleFormat> for hound::SampleFormat {
    fn from(v: self::SampleFormat) -> Self {
        match v {
            self::SampleFormat::Float => hound::SampleFormat::Float,
            self::SampleFormat::Int => hound::SampleFormat::Int,
        }
    }
}
