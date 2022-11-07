use tinyoas::util::errors::CliResult;
use crate::command_prelude::*;

pub fn main() -> CliResult {
    let matches = cli().get_matches();
    match matches.subcommand() {
        _ => unreachable!()
    }
}


fn cli() -> App {
    App::new("tinyoas")
        .about("CLI toolbox for tinyOAS REST API specification documents")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(clap::crate_version!())
        .override_usage("tinyoas <COMMAND> <COMMAND_OPTIONS>")
}
