use clap::Command;
use tinyoas::CliResult;
use tinyoas::commands::{build, validate};

const BUILD: &str = "build";
const VALIDATE: &str = "validate";


pub fn main() -> CliResult {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some((BUILD, _)) => {
            build()
        }
        Some((VALIDATE, _)) => {
            validate()
        }
        _ => unreachable!()
    }
}


fn cli() -> Command<'static> {
    Command::new("tinyoas")
        .about("CLI toolbox for tinyOAS REST API specification documents")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(clap::crate_version!())
        .override_usage("tinyoas <COMMAND> <COMMAND_OPTIONS>")

        .subcommand(
            Command::new(BUILD)
                .about("Builds OAS 3 specification document from tinyOAS source")
                .alias("b")
        )
        .subcommand(
            Command::new(VALIDATE)
                .about("Performs validation of tinyOAS source documents")
        )
}
