use std::fs;
use std::io;
use untex::deps::write_file_deps;
//use untex::explain::explain_file;

use clap::{Arg, Command};

enum Writer {
    File(fs::File),
    Stdout(io::Stdout),
}

impl io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Writer::File(file) => file.write(buf),
            Writer::Stdout(stdout) => stdout.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            Writer::File(file) => file.flush(),
            Writer::Stdout(stdout) => stdout.flush(),
        }
    }
}

pub fn main() {
    let matches = Command::new("UnTeX")
        .version("0.2.0-alpha")
        .author("Jérome Eertmans <jeertmans@icloud.com>")
        .about("Understand and manipulate TeX files.")
        .arg(
            Arg::new("output")
                .value_name("file")
                .short('o')
                .long("output")
                .help("Place the output into <file>")
                )
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
        .subcommand(
            Command::new("deps")
            .about("Write the list dependencies of a main TeX file in a tree format. This includes other TeX documents, images, bibiographies or data files")
            .arg(
                Arg::new("FILE")
                    .help("Set the input file to use")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("grouped")
                .short('g')
                .takes_value(false)
                .help("Group files by kind")
                ),

        )
        .get_matches();

    let writer = match matches.value_of("output") {
        Some(file) => Writer::File(fs::File::create(file).unwrap()),
        _ => Writer::Stdout(io::stdout()),
    };

    match matches.subcommand() {
        Some(("explain", sub_matches)) => {
            let filename = sub_matches.value_of("FILE").unwrap();
            let verbose = sub_matches.is_present("verbose");
            //explain_file(filename, verbose);
        }
        Some(("deps", sub_matches)) => {
            let filename = sub_matches.value_of("FILE").unwrap();
            let grouped = sub_matches.is_present("grouped");
            write_file_deps(filename, writer, grouped).unwrap()
        }
        _ => unreachable!(),
    }
}
