use std::path::PathBuf;
use clap::{App, Command};
use tinyoas::CliResult;
use tinyoas::commands::{build, inspect};

const BUILD: &str = "build";
const INSPECT: &str = "inspect";


pub fn main() -> CliResult {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some((BUILD, options)) => {
            let source = options.path_arg("source");
            let target = options.path_arg("target");
            build(source, target)
        }
        Some((INSPECT, options)) => {
            let source = options.path_arg("source");
            inspect(source)
        }
        _ => unreachable!()
    }
}


fn cli() -> App<'static> {
    Command::new("tinyoas")
        .about("CLI toolbox for tinyOAS REST API specification documents")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(clap::crate_version!())
        .override_usage("tinyoas <COMMAND> <OPTIONS>")

        .subcommand(
            clap::command!(BUILD)
                .about("Build OpenAPI 3 specification documents from tinyOAS source")
                .alias("b")
                .args([
                    source_path_arg(),
                    target_path_arg()
                ])
        )
        .subcommand(
            Command::new(INSPECT)
                .alias("i")
                .about("Inspect tinyOAS source documents")
                .args([source_path_arg()])
        )
}


fn source_path_arg() -> clap::Arg<'static> {
    clap::arg!(-s --source <PATH> "A path to tinyOAS source documents")
        .required(false)
        .default_value(".")
        .value_parser(clap::value_parser!(PathBuf))
}


fn target_path_arg() -> clap::Arg<'static> {
    clap::arg!(-t --target <PATH> "A path to built OpenAPI 3 specification documents")
        .required(false)
        .default_value(".")
        .value_parser(clap::value_parser!(PathBuf))
}


trait PathArgs {
    fn path_arg(&self, arg_name: &str) -> &PathBuf;
}

impl PathArgs for &clap::ArgMatches {
    fn path_arg(&self, arg_name: &str) -> &PathBuf {
        self.get_one::<PathBuf>(arg_name).unwrap()
    }    
}
