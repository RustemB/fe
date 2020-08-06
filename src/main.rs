use clap::{crate_version, App, Arg};
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
};
mod data_types;

fn main() -> Result<(), String> {
    let cli = gen_cli();

    let mut user_input = String::new();
    match cli.value_of("input") {
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

    let data_format =
        data_format_to_enum(cli.value_of("read_format").unwrap(), user_input).unwrap();

    print_data(
        data_format,
        cli.is_present("uglify"),
        cli.value_of("output"),
    );

    Ok(())
}

fn data_format_to_enum(
    format: &str,
    data_src: String,
) -> Result<data_types::formats::DataFormats, String> {
    match format.to_lowercase().as_str() {
        "json" => match serde_json::from_str(&data_src) {
            Ok(val) => Ok(data_types::formats::DataFormats::Json(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "yaml" => match serde_yaml::from_str(&data_src) {
            Ok(val) => Ok(data_types::formats::DataFormats::Yaml(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "ron" => match ron::from_str::<ron::Value>(&data_src) {
            Ok(val) => Ok(data_types::formats::DataFormats::Ron(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        "toml" => match toml::from_str(&data_src) {
            Ok(val) => Ok(data_types::formats::DataFormats::Toml(val)),
            Err(e) => Err(format!("Something went wrong! {}", e)),
        },
        _ => Err("Unreachable zone!".to_owned()),
    }
}

fn print_data(
    data_type: data_types::formats::DataFormats,
    is_ugly: bool,
    file_to_write: Option<&str>,
) {
    let string = match data_type {
        data_types::formats::DataFormats::Json(data_src) => {
            if is_ugly {
                serde_json::to_string(&data_src).unwrap()
            } else {
                serde_json::to_string_pretty(&data_src).unwrap()
            }
        }
        data_types::formats::DataFormats::Yaml(data_src) => {
            serde_yaml::to_string(&data_src).unwrap()
        }
        data_types::formats::DataFormats::Ron(data_src) => {
            if is_ugly {
                ron::to_string(&data_src).unwrap()
            } else {
                ron::ser::to_string_pretty(&data_src, ron::ser::PrettyConfig::default()).unwrap()
            }
        }
        data_types::formats::DataFormats::Toml(data_src) => {
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

fn gen_cli() -> clap::ArgMatches<'static> {
    App::new("fe")
        .version(crate_version!())
        .author("RustemB <bakirov.com@yandex.ru>")
        .about("JSON (other in next versions) manipulator")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FILE")
                .help("Input file to manipulate"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .value_name("FILE")
                .help("Output file"),
        )
        .arg(
            Arg::with_name("uglify")
                .short("u")
                .long("uglify")
                .takes_value(false)
                .help("Uglify data"),
        )
        .arg(
            Arg::with_name("read_format")
                .short("f")
                .long("format")
                .takes_value(true)
                .value_name("FORMAT")
                .default_value("json")
                .case_insensitive(true)
                .possible_values(&["json", "yaml", "ron", "toml"])
                .help("Input data format"),
        )
        //.arg(Arg::with_name("query").last(true).default_value("/"))
        .get_matches()
}
