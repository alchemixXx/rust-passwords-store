use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    /// Add new password to the password store
    Add {
        #[arg(short, long)]
        service: String,

        #[arg(short, long)]
        login: String,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        comment: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },
    /// Get password from the password store
    Get {
        #[arg(short, long)]
        service: String,
    },
    /// Get all passwords from the password store
    GetAll {},
    /// Remove password from the password store
    Remove {
        #[arg(short, long)]
        service: String,
    },
    /// Update password in the password store
    Update {
        #[arg(short, long)]
        service: String,

        #[arg(short, long)]
        login: Option<String>,

        #[arg(short, long)]
        password: Option<String>,

        #[arg(short, long)]
        comment: Option<String>,

        #[arg(short, long)]
        username: Option<String>,
    },
    /// Generate a new password
    Generate {},
}
