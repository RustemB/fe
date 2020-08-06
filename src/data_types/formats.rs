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
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "yaml" => match serde_yaml::from_str(&data_src) {
            Ok(val) => Ok(DataFormats::Yaml(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "ron" => match ron::from_str::<ron::Value>(&data_src) {
            Ok(val) => Ok(DataFormats::Ron(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "toml" => match toml::from_str(&data_src) {
            Ok(val) => Ok(DataFormats::Toml(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        _ => Err("Unreachable zone!".to_owned()),
    }
}
