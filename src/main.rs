mod cli;
mod data_types;
use data_types::formats;
use std::path;
use std::{
    fs::File,
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
                extension = path::Path::new(f)
                    .extension()
                    .map(|x| x.to_str().unwrap_or(""));
            }
            _ => {
                return Err(format!("File `{}' not exist.", f));
            }
        },
        None => {
            if let Err(x) = io::stdin().read_to_string(&mut user_input) {
                return Err(format!("Something went wrong: {}", x));
            }
        }
    }

    let is_color = match fe_cli.value_of("color") {
        Some("always") => true,
        Some("never") => false,
        Some("auto") => option_env!("NO_COLOR").is_none(),
        _ => return Err("Unreachable zone!".to_owned()),
    };

    formats::print_data(
        formats::data_format_to_enum(
            extension.unwrap_or_else(|| fe_cli.value_of("read_format").unwrap()),
            user_input,
        )
        .unwrap(),
        fe_cli.is_present("uglify"),
        fe_cli.value_of("output"),
        is_color,
    );

    Ok(())
}
