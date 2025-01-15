mod cli;
mod storage;
use clap::Parser;
use cli::{Cli, Commands};
use storage::Storage;
// use uuid::Uuid;
fn main() {
    let cli_args = Cli::parse();
    println!("Hello, world!");

    let storage = Storage::new();

    println!("{:?}", storage);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli_args.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Add { add }) => {
            if *add {
                println!("Adding new password...");
            } else {
                println!("Not adding new password...");
            }
        }
        Some(_) => {
            println!("Not implemented yet...");
        }
        None => {}
    }
}
