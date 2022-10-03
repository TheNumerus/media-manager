use crate::AppError;
use std::fmt::Debug;
use std::fs::{File, OpenOptions};
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

        if let Ok(config) = Self::try_from(conf) {
            return Ok(config);
        }

        println!("Please input your TMDB token:");
        let token = crate::input::read_line()?;

        let config = Self { tmdb_token: token };
        config.write_to_file(file)?;

        Ok(config)
    }

    fn write_to_file(&self, mut file: File) -> Result<(), AppError> {
        let toml = toml::to_string(&Value::from(self)).expect("Failed to create config");

        file.write_all(toml.as_bytes())
            .map_err(|_| AppError::Config("Failed to write config file".into()))
    }
}

impl TryFrom<Table> for Config {
    type Error = AppError;

    fn try_from(table: Table) -> Result<Self, Self::Error> {
        if let Some(token) = table.get("tmdb_token") {
            return match token {
                Value::String(s) => Ok(Self {
                    tmdb_token: s.clone(),
                }),
                _ => Err(AppError::Config("Invalid data type of `tmdb_token`".into())),
            };
        } else {
            Err(AppError::Config("Missing value `tmdb_token`".into()))
        }
    }
}

impl From<&Config> for Value {
    fn from(config: &Config) -> Self {
        let Config { tmdb_token } = config;

        let mut config_toml = Table::with_capacity(1);
        config_toml.insert("tmdb_token".into(), Value::String(tmdb_token.clone()));

        Value::Table(config_toml)
    }
}
