use untex::explain::explain_file;

use clap::{Arg, Command};

pub fn main() {
    let matches = Command::new("UnTeX")
        .version("0.1.1-alpha")
        .author("Jérome Eertmans <jeertmans@icloud.com>")
        .about("Understand and manipulate TeX files.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("explain")
            .about("Give an internal explanation of a file. Useful to see how UnTeX understands TeX files")
            .arg(
                Arg::new("FILE")
                    .help("Set the input file to use")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("verbose")
                .short('v')
                .long("verbose")
                .takes_value(false)
                .help("Use verbose output")
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("explain", sub_matches)) => {
            let filename = sub_matches.value_of("FILE").unwrap();
            let verbose = sub_matches.is_present("verbose");
            explain_file(filename, verbose);
        }
        _ => unreachable!(),
    }
}
