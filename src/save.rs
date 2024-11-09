use bevy::prelude::Resource;
// src/save.rs
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Resource)]
pub struct SaveFileData {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub created_at: String,
}

impl Default for SaveFileData {
    fn default() -> Self {
        Self {
            id: String::new(),
            width: 32,
            height: 32,
            created_at: chrono::Local::now().to_string(),
        }
    }
}

pub fn save_universe(dimensions: (u32, u32)) -> Result<SaveFileData, String> {
    fs::create_dir_all("saves").map_err(|e| e.to_string())?;

    let timestamp = Utc::now().timestamp();
    let id = format!("universe_{}", timestamp);

    let universe_data = SaveFileData {
        id: id.clone(),
        width: dimensions.0,
        height: dimensions.1,
        created_at: Utc::now().to_rfc3339(),
    };

    let file_path = PathBuf::from("saves").join(format!("{}.json", id));
    let json = serde_json::to_string_pretty(&universe_data).map_err(|e| e.to_string())?;
    fs::write(file_path, json).map_err(|e| e.to_string())?;

    Ok(universe_data)
}
