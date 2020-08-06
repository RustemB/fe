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
    match fe_cli.value_of("input") {
        Some(f) => match File::open(f) {
            Ok(fi) => {
                let mut reader = BufReader::new(fi);
                if let Err(x) = reader.read_to_string(&mut user_input) {
                    return Err(format!("Problem with reading file `{}': {}", f, x));
                }
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

    //let query_of_data = cli.value_of("query").unwrap_or_else(|| {
    //    println!("Something went wrong: plz spcfy qry");
    //    process::exit(1);
    //});

    //parsed_data = parsed_data.pointer(query_of_data).unwrap_or_else(|| {
    //    println!("Something went wrong: no query");
    //    process::exit(1);
    //});

    print_data(
        formats::data_format_to_enum(fe_cli.value_of("read_format").unwrap(), user_input).unwrap(),
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
