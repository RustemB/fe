pub enum DataFormats {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    Ron(ron::Value),
    Toml(toml::Value),
    //Lexpr(Value),
    //MsgPack(Value),
    //Csv(Value),
    //Tsv(Value),
    //Wasm(Value),
    //Cson(Value),
}

pub fn data_format_to_enum(format: &str, data_src: String) -> Result<DataFormats, String> {
    match format.to_lowercase().as_str() {
        "json" => match serde_json::from_str(&data_src) {
            Ok(val) => Ok(DataFormats::Json(val)),
            Err(e) => Err(format!("Something went wrong: {}", e)),
        },
        "yaml" | "yml" => match serde_yaml::from_str(&data_src) {
            Ok(val) => Ok(DataFormats::Yaml(val)),
            Err(e) => Err(format!("Something went wrong: {}", e)),
        },
        "ron" => match ron::from_str::<ron::Value>(&data_src) {
            Ok(val) => Ok(DataFormats::Ron(val)),
            Err(e) => Err(format!("Something went wrong: {}", e)),
        },
        "toml" => match toml::from_str(&data_src) {
            Ok(val) => Ok(DataFormats::Toml(val)),
            Err(e) => Err(format!("Something went wrong: {}", e)),
        },
        _ => Err("Unreachable zone...".to_owned()),
    }
}

pub fn print_data(data_type: DataFormats, is_ugly: bool, file_to_write: Option<&str>) {
    let string = match data_type {
        DataFormats::Json(data_src) => {
            if is_ugly {
                serde_json::to_string(&data_src).unwrap()
            } else {
                serde_json::to_string_pretty(&data_src).unwrap()
            }
        }
        DataFormats::Yaml(data_src) => serde_yaml::to_string(&data_src).unwrap(),
        DataFormats::Ron(data_src) => {
            if is_ugly {
                ron::to_string(&data_src).unwrap()
            } else {
                ron::ser::to_string_pretty(&data_src, ron::ser::PrettyConfig::default()).unwrap()
            }
        }
        DataFormats::Toml(data_src) => {
            if is_ugly {
                toml::to_string(&data_src).unwrap()
            } else {
                toml::to_string_pretty(&data_src).unwrap()
            }
        }
    };
    match file_to_write {
        Some(file) => std::fs::write(file, string).expect("Problems with writing to file."),
        None => println!("{}", string),
    }
}
