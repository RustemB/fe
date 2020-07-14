use clap::{App, Arg};
use serde_json;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    process,
};

fn main() {
    let cli = App::new("fe")
        .version("0.1.1")
        .author("RustemB <bakirov.com@yandex.ru>")
        .about("JSON (other in next versions) manipulator.")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .value_name("FILE")
                .help("Input file to manipulate."),
        )
        .arg(
            Arg::with_name("uglify")
                .short("u")
                .long("uglify")
                .takes_value(false)
                .help("Uglify data."),
        )
        .get_matches();
    let mut user_input = String::new();
    match cli.value_of("input") {
        Some(f) => {
            let file = File::open(f);
            match file {
                Ok(fi) => {
                    let mut reader = BufReader::new(fi);
                    if let Err(x) = reader.read_to_string(&mut user_input) {
                        println!("Problem with reading file `{}': {}", f, x);
                        process::exit(1);
                    }
                }
                Err(_) => {
                    println!("File `{}' not exist.", f);
                    process::exit(1);
                }
            }
        }
        None => match io::stdin().read_to_string(&mut user_input) {
            Ok(_) => {}
            Err(x) => {
                println!("Something went wrong! {}", x);
                process::exit(1);
            }
        },
    }
    let v: serde_json::Value = match serde_json::from_str(&user_input) {
        Ok(n) => n,
        Err(x) => {
            println!("Something went wrong! {}", x);
            process::exit(1);
        }
    };
    let output_data = if cli.is_present("uglify") {
        match serde_json::to_string(&v) {
            Ok(n) => n,
            Err(x) => {
                println!("Something went wrong: {}", x);
                process::exit(1);
            }
        }
    } else {
        match serde_json::to_string_pretty(&v) {
            Ok(n) => n,
            Err(x) => {
                println!("Something went wrong: {}", x);
                process::exit(1);
            }
        }
    };
    println!("{}", output_data);
}
