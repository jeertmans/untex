use clap::Parser;
use untex::cli::*;

pub fn main() {
    let cli = Cli::parse_from(wild::args());

    match cli.command {
        Command::Check => unimplemented!(),
        Command::Dependencies => unimplemented!(),
        Command::Expand => unimplemented!(),
        Command::Highlight(cmd) => cmd.execute().unwrap(),
        Command::Format => unimplemented!(),
        Command::Parse => unimplemented!(),
        Command::Complete(cmd) => cmd.execute().unwrap(),
    }
}
