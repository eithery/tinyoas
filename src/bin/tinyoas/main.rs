mod cli;

fn main() {
    let result = cli::main();
    match result {
        Ok(()) => {}
        Err(_) => {} // TODO: Implement exit with error
    }
}
