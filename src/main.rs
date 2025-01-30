mod cli;
mod custom_result;
mod encrypt;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use custom_result::CustomResult;
use encrypt::Encrypt;
use storage::Storage;
fn main() -> CustomResult<()> {
    // Example key and IV (must be 32 bytes and 12 bytes respectively)
    let secret_key = "0123456789abcdef0123456789abcd11";
    let initialization_vector = "000102030405060708090a0b"; // 12 bytes
    let path: &str = "src/data/data.json";
    let storage = Storage::new(path);
    let encrypt = Encrypt::new(secret_key, initialization_vector);

    // Plaintext to encrypt
    let plaintext = "Hello, Rust!";

    // Encrypt the plaintext
    let ciphertext = encrypt.encrypt(plaintext)?;
    println!("Encrypted: {}", ciphertext);

    // Decrypt the ciphertext
    let decrypted = encrypt.decrypt(&ciphertext)?;
    println!("Decrypted: {}", decrypted);

    let cli_args = Cli::parse();
    println!("Hello, world!");

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
    Ok(())
}
