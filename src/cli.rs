use clap::{crate_version, App, Arg, ArgMatches};

pub fn gen_cli() -> ArgMatches<'static> {
    App::new("fe")
        .version(crate_version!())
        .author("RustemB <bakirov.com@yandex.ru>")
        .about("Data format manipulator")
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
        .arg(
            Arg::with_name("color")
                .long("color")
                .takes_value(true)
                .value_name("WHEN")
                .help("Coloring")
                .case_insensitive(true)
                .default_value("auto")
                .hide_default_value(true)
                .possible_values(&["auto", "always", "never"]),
        )
        //.arg(Arg::with_name("query").last(true).default_value("/"))
        .get_matches()
}
