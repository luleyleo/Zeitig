
use std::path::{Path, PathBuf};
use directories::ProjectDirs;

static FILE_NAME: &str = "zeitig.db";

pub fn data_file() -> PathBuf {
    if cfg!(debug_assertions) {
        log::info!("Accessing debug data file.");
        return PathBuf::from(FILE_NAME);
    }
    if let Some(pd) = ProjectDirs::from("", "", "Zeitig") {
        let data = pd.data_dir();
        if std::fs::create_dir_all(data).is_ok() {
            return data.join(FILE_NAME);
        }
    }
    Path::new(FILE_NAME).to_owned()
}
