use directories::ProjectDirs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::AppError;

const DEFAULT_DB_NAME: &str = "data.db";
const DEFAULT_CONFIG_NAME: &str = "config.toml";

pub struct Paths {
    pub db_path: PathBuf,
    pub config_path: PathBuf,
}

impl Paths {
    pub fn new() -> Result<Self, AppError> {
        match ProjectDirs::from("", "", "media-manager") {
            None => Err(AppError::Input(
                "No home directory found".to_owned(),
                ErrorKind::Other.into(),
            )),
            Some(dirs) => {
                let mut db_path = dirs.data_dir().to_owned();
                let mut config_path = dirs.config_dir().to_owned();

                db_path.push(DEFAULT_DB_NAME);
                config_path.push(DEFAULT_CONFIG_NAME);

                Ok(Self {
                    db_path,
                    config_path,
                })
            }
        }
    }

    pub fn make_dirs(&self) -> Result<(), AppError> {
        let db_folder = self.db_path.parent().unwrap();
        std::fs::create_dir_all(db_folder)
            .map_err(|e| AppError::Input("Could not create folder for data".to_owned(), e))?;

        let config_folder = self.config_path.parent().unwrap();
        std::fs::create_dir_all(config_folder).map_err(|e| {
            AppError::Input("Could not create folder for configuration".to_owned(), e)
        })?;

        Ok(())
    }
}
