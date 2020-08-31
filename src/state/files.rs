use directories::ProjectDirs;
use std::path::{Path, PathBuf};

use crate::state::AppState;

// TODO: Consider removing those
#[allow(dead_code)]
pub fn read_state() -> AppState {
    let path = data_file_path();
    if path.exists() {
        let data = std::fs::read(path).expect("Failed to read data.");
        rmp_serde::from_slice(&data).expect("Failed to deserialize data.")
    } else {
        AppState::default()
    }
}

// TODO: Consider removing those
#[allow(dead_code)]
pub fn write_state(state: AppState) {
    let path = data_file_path();
    let data = rmp_serde::to_vec(&state).expect("Failed to serialize data.");
    std::fs::write(path, &data).expect("Failed to write data.");
}

pub fn data_file_path() -> PathBuf {
    if cfg!(debug_assertions) {
        log::info!("Accessing debug data file.");
        return PathBuf::from("./zeitig.db");
    }
    if let Some(pd) = ProjectDirs::from("", "", "Zeitig") {
        let data = pd.data_dir();
        if std::fs::create_dir_all(data).is_ok() {
            return data.join("zeitig.db");
        }
    }
    Path::new("zeitig.mp").to_owned()
}
