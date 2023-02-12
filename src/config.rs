use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, process};

const APP_NAME: &str = "rmail";
const CONFIG_FILE: &str = "config.json";

fn get_config_path() -> PathBuf {
    let config_path = match dirs::config_dir() {
        Some(config_dir) => config_dir,
        _ => panic!("Could not determine the config directory of your user."),
    };

    let config_file = Path::new(&config_path).join(APP_NAME).join(CONFIG_FILE);

    return config_file.to_path_buf();
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub domain: String,
}

pub trait ConfigFile {
    fn load() -> Config;
    fn save(&self);
}

impl ConfigFile for Config {
    fn load() -> Config {
        let config_path = get_config_path();
        let json_data = match fs::read_to_string(&config_path) {
            Ok(data) => data,
            Err(_) => {
                println!("No config file found. Run `rmail init` to create one.");
                process::exit(-1);
            }
        };

        let config: Config = match serde_json::from_str(&json_data) {
            Ok(config) => config,
            Err(_why) => panic!("Seems like your config file is damaged."),
        };
        return config;
    }

    fn save(&self) {
        let json_data = match serde_json::to_string(self) {
            Ok(data) => data,
            Err(why) => panic!("Could not serialize your config: {}", why),
        };

        let config_file = get_config_path();
        let path = Path::new(&config_file)
            .parent()
            .expect("get_config_path ensures that the path is not a root path or is empty.");

        let _ = fs::create_dir_all(path);

        match fs::write(get_config_path(), json_data) {
            Ok(_) => (),
            Err(why) => panic!("Could not write config: {}", why),
        }
    }
}
