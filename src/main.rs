use clap::{crate_version, App, Arg};
use serde_json;
use std::{
    fs::{self, File},
    io::{self, BufReader, Read},
    process,
};

fn main() {
    let cli = App::new("fe")
        .version(crate_version!())
        .author("RustemB <bakirov.com@yandex.ru>")
        .about("JSON (other in next versions) manipulator")
        .version_short("v")
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
        .get_matches();
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
    let parsed_data: serde_json::Value = match serde_json::from_str(&user_input) {
        Ok(n) => n,
        Err(x) => {
            println!("Something went wrong! {}", x);
            process::exit(1);
        }
    };
    let method_to_print = if cli.is_present("uglify") {
        serde_json::to_string
    } else {
        serde_json::to_string_pretty
    };
    let output_data = match method_to_print(&parsed_data) {
        Ok(n) => n,
        Err(x) => {
            println!("Something went wrong: {}", x);
            process::exit(1);
        }
    };
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
