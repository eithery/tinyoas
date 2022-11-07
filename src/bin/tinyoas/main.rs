use tinyoas::util::command_prelude;

mod cli;
mod commands;

fn main() {
    let result = cli::main();
    match result {
        Ok(()) => {}
        Err(_) => {} // TODO: Implement exit with error
    }
}
