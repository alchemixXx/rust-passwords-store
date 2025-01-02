mod cli;
use clap::Parser;
use cli::{Cli, Commands};
fn main() {
    let cli_args = Cli::parse();
    println!("Hello, world!");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli_args.command {
        Some(Commands::Add {
            service,
            username,
            email,
            password,
        }) => {
            println!(
                "Adding new password for service '{}', username '{}', email '{}', password '{}'",
                service,
                username.as_deref().unwrap_or("None"),
                email,
                password
            );
        }
        Some(Commands::Get { get }) => {
            if *get {
                println!("Getting password...");
            } else {
                println!("Not getting password...");
            }
        }
        Some(Commands::GetAll {}) => {
            println!("Getting all passwords..., this is a test");
        }
        Some(Commands::Update { update }) => {
            if *update {
                println!("Updating password...");
            } else {
                println!("Not updating password...");
            }
        }
        Some(Commands::Remove { remove }) => {
            if *remove {
                println!("Removing  password...");
            } else {
                println!("Not removing password...");
            }
        }
        Some(Commands::Generate { generate }) => {
            if *generate {
                println!("Generating password...");
            } else {
                println!("Not generating password...");
            }
        }
        None => {}
    }
}
