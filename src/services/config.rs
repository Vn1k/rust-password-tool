use crate::config::APP_NAME;
use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::models::Config;
use std::io;

pub fn config_path() -> PathBuf {
    let project =
        ProjectDirs::from("com", "vinik", APP_NAME).expect("Cannot find config directory");
    let dir = project.config_dir();

    fs::create_dir_all(dir).expect("Failed to create config directory");

    dir.join("config.toml")
}

fn default_config() -> Config {
    Config {
        length: 12,
        uppercase: true,
        digits: true,
        symbols: true,
    }
}

pub fn load_config() -> Config {
    let path = config_path();

    if !path.exists() {
        let cfg = default_config();
        save_config(&cfg).expect("Failed to save default config");
        return cfg;
    }

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {:?}", path));

    toml::from_str::<Config>(&content)
        .unwrap_or_else(|_| panic!("Failed to parse config file at {:?}", path))
}

pub fn save_config(config: &Config) -> Result<(), io::Error> {
    let path = config_path();
    let toml_str = toml::to_string_pretty(config).expect("Failed to serialize config to TOML");

    fs::write(path, toml_str)
}
