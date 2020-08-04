/// Data formats that available in `fe`, but should be replaced with some modularity
pub enum DataFormats {
    Json(serde_json::Value),
    Yaml(serde_yaml::Value),
    /// Prints strange json :thinking:
    Ron(ron::Value),
    //Lexpr(Value),
    //MsgPack(Value),
    //Csv(Value),
    //Tsv(Value),
    //Wasm(Value),
    //Cson(Value),
}
