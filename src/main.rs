mod error;
mod file;
mod cli;

use cli::CLI;
use std::process;

fn main() {
    let mut cli = CLI::new();
    
    if let Err(e) = cli.run() {
        eprintln!("Fatal error: {}", e);
        process::exit(1);
    }
}
