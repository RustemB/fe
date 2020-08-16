use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

// First-ly look at global config, then on individual
#[derive(Deserialize)]
pub struct Config {
    pub global: Option<Global>,
    pub json: Option<Json>,
    pub yaml: Option<Yaml>,
    pub toml: Option<Toml>,
    pub ron: Option<Ron>,
}

#[derive(Deserialize)]
pub struct Global {
    pub uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Json {
    pub uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Yaml {
    pub uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Toml {
    pub uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Ron {
    pub uglify: Option<bool>,
}

pub fn get_config() -> Option<Config> {
    if let Some(project_dir) = directories::ProjectDirs::from("", "", "fe") {
        let config_dir = project_dir.config_dir();
        let config_file = config_dir.join("config.toml");
        if let Ok(file) = File::open(config_file) {
            let mut buf = String::new();
            let mut bufreader = BufReader::new(file);
            match bufreader.read_to_string(&mut buf) {
                Ok(_) => match toml::from_str::<Config>(&buf) {
                    Ok(val) => Some(val),
                    _ => None,
                },
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}
