mod cli;

fn main() {
    match cli::main() {
        Ok(()) => {},
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
