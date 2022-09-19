use crate::AppError;
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use toml::value::Table;
use toml::Value;

pub struct Config {
    pub tmdb_token: String,
}

impl Config {
    pub fn init(path: impl AsRef<Path> + Debug) -> Result<Self, AppError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
            .map_err(|e| {
                AppError::Input(format!("Could not open config file at {:?}", &path), e)
            })?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            AppError::Input(format!("Could not read config file at  {:?}", path), e)
        })?;

        let conf = toml::from_str::<Table>(&contents)
            .map_err(|_| AppError::Config("Invalid configuration format".into()))?;

        if let Some(token) = conf.get("tmdb_token") {
            return match token {
                Value::String(s) => Ok(Self {
                    tmdb_token: s.clone(),
                }),
                _ => Err(AppError::Config("Invalid data type of `tmdb_token`".into())),
            };
        }

        let token = Self::get_token()?;

        let mut config = Table::new();
        config.insert("tmdb_token".into(), Value::String(token.clone()));
        let toml = toml::to_string(&config)
            .map_err(|_| AppError::Config("Failed to create config".into()))?;
        file.write_all(toml.as_bytes())
            .map_err(|_| AppError::Config("Failed to write config file".into()))?;

        Ok(Self { tmdb_token: token })
    }

    fn get_token() -> Result<String, AppError> {
        println!("Please input your TMDB token:");

        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|e| AppError::Input("Failed to read input".into(), e))?;

        buf.trim();

        Ok(buf)
    }
}
