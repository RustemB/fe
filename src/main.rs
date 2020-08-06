mod cli;
mod data_types;
use data_types::formats;
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
};

fn main() -> Result<(), String> {
    let fe_cli = cli::gen_cli();

    let mut user_input = String::new();
    let mut extension: Option<&str> = None;
    match fe_cli.value_of("input") {
        Some(f) => match File::open(f) {
            Ok(fi) => {
                let mut reader = BufReader::new(fi);
                if let Err(x) = reader.read_to_string(&mut user_input) {
                    return Err(format!("Problem with reading file `{}': {}", f, x));
                }
                let ext: Vec<&str> = f.rsplit('.').collect();
                extension = ext.first().map(|x| *x);
            }
            _ => {
                return Err(format!("File `{}' not exist.", f));
            }
        },
        None => {
            if let Err(x) = io::stdin().read_to_string(&mut user_input) {
                return Err(format!("Something went wrong! {}", x));
            }
        }
    }

    print_data(
        formats::data_format_to_enum(
            extension.unwrap_or(fe_cli.value_of("read_format").unwrap()),
            user_input,
        )
        .unwrap(),
        fe_cli.is_present("uglify"),
        fe_cli.value_of("output"),
    );

    Ok(())
}

fn print_data(data_type: formats::DataFormats, is_ugly: bool, file_to_write: Option<&str>) {
    let string = match data_type {
        formats::DataFormats::Json(data_src) => {
            if is_ugly {
                serde_json::to_string(&data_src).unwrap()
            } else {
                serde_json::to_string_pretty(&data_src).unwrap()
            }
        }
        formats::DataFormats::Yaml(data_src) => serde_yaml::to_string(&data_src).unwrap(),
        formats::DataFormats::Ron(data_src) => {
            if is_ugly {
                ron::to_string(&data_src).unwrap()
            } else {
                ron::ser::to_string_pretty(&data_src, ron::ser::PrettyConfig::default()).unwrap()
            }
        }
        formats::DataFormats::Toml(data_src) => {
            if is_ugly {
                toml::to_string(&data_src).unwrap()
            } else {
                toml::to_string_pretty(&data_src).unwrap()
            }
        }
    };
    match file_to_write {
        Some(file) => fs::write(file, string).expect("Problems with writing to file!"),
        None => println!("{}", string),
    }
}
