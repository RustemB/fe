use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;

// First-ly look at global config, then on individual
#[derive(Deserialize)]
pub struct Config {
    global: Option<Global>,
    json: Option<Json>,
    yaml: Option<Yaml>,
    toml: Option<Toml>,
    ron: Option<Ron>,
}

#[derive(Deserialize)]
pub struct Global {
    uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Json {
    uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Yaml {
    uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Toml {
    uglify: Option<bool>,
}

#[derive(Deserialize)]
pub struct Ron {
    uglify: Option<bool>,
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
