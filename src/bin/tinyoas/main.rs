mod cli;

fn main() {
    if let Err(err) = cli::main() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
