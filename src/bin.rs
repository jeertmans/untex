use clap::Parser;
use untex::cli::*;
use untex::error::Result;

macro_rules! issue {
    ($issue:expr) => {
        build_cli().error(clap::error::ErrorKind::InvalidValue, format!("The command you are trying to access is currently unimplemented, and its development can be followed on https://github.com/jeertmans/untex/pull/{}", $issue)).exit()
    };
}

pub fn main() -> Result<()> {
    let cli = Cli::parse_from(wild::args());

    match cli.command {
        Command::Check => issue!(7),
        Command::Dependencies => issue!(8),
        Command::Expand => issue!(9),
        Command::Highlight(cmd) => cmd.execute()?,
        Command::Format(cmd) => cmd.execute()?,
        Command::Parse => issue!(11),
        #[cfg(feature = "cli-complete")]
        Command::Completions(cmd) => cmd.execute()?,
    }

    Ok(())
}
