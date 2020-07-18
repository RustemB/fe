use clap::{crate_version, App, Arg};
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
    process,
};
mod formats;

fn main() {
    let cli = gen_cli();

    let mut user_input = String::new();
    match cli.value_of("input") {
        Some(f) => match File::open(f) {
            Ok(fi) => {
                let mut reader = BufReader::new(fi);
                if let Err(x) = reader.read_to_string(&mut user_input) {
                    println!("Problem with reading file `{}': {}", f, x);
                    process::exit(1);
                }
            }
            _ => {
                println!("File `{}' not exist.", f);
                process::exit(1);
            }
        },
        None => {
            if let Err(x) = io::stdin().read_to_string(&mut user_input) {
                println!("Something went wrong! {}", x);
                process::exit(1);
            }
        }
    }

    let parsed_data: serde_json::Value = serde_json::from_str(&user_input).unwrap_or_else(|x| {
        println!("Something went wrong! {}", x);
        process::exit(1);
    });

    //let query_of_data = cli.value_of("query").unwrap_or_else(|| {
    //    println!("Something went wrong: plz spcfy qry");
    //    process::exit(1);
    //});

    //parsed_data = parsed_data.pointer(query_of_data).unwrap_or_else(|| {
    //    println!("Something went wrong: no query");
    //    process::exit(1);
    //});

    let data_format = cli.value_of("read_format").unwrap();
    let method_to_print = printing_function(
        data_format_to_enum(data_format).unwrap_or(formats::formats::DataFormats::Json),
        cli.is_present("uglify"),
    )
    .unwrap();

    let output_data = method_to_print(&parsed_data).unwrap_or_else(|x| {
        println!("Something went wrong: {}", x);
        process::exit(1);
    });

    match cli.value_of("output") {
        Some(n) => {
            if let Err(x) = fs::write(n, output_data) {
                println!("Something went wrong: {}", x);
                process::exit(1);
            }
        }
        None => println!("{}", output_data),
    }
}

fn data_format_to_enum(format: &str) -> Result<formats::formats::DataFormats, ()> {
    match format {
        "json" => Ok(formats::formats::DataFormats::Json),
        _ => Err(()),
    }
}

fn printing_function<T>(
    data_type: formats::formats::DataFormats,
    is_ugly: bool,
) -> Option<fn(&T) -> serde_json::Result<String>>
where
    T: serde::ser::Serialize,
{
    match data_type {
        formats::formats::DataFormats::Json => Some(if is_ugly {
            serde_json::to_string
        } else {
            serde_json::to_string_pretty
        }),
        //_ => None,
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
                .possible_values(&["json"])
                .help("Input data format"),
        )
        //.arg(Arg::with_name("query").last(true).default_value("/"))
        .get_matches()
}
