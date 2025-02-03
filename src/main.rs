mod cli;
mod cli_processor;
mod custom_result;
mod encrypt;
mod storage;

use clap::Parser;
use cli::Cli;
use cli_processor::CliProcessor;
use custom_result::CustomResult;

fn main() -> CustomResult<()> {
    // Example key and IV (must be 32 bytes and 12 bytes respectively)
    let secret_key = "0123456789abcdef0123456789abcd11";
    let initialization_vector = "000102030405060708090a0b"; // 12 bytes
    let path: &str = "src/data/data.json";

    let cli_args = Cli::parse();
    println!("{:#?}", cli_args);
    let cli_processor = CliProcessor::new(
        secret_key.to_string(),
        initialization_vector.to_string(),
        path.to_string(),
    );

    cli_processor.process(cli_args)?;

    Ok(())
}
