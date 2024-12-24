use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Required name to operate on
    pub name: String,

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
        add: bool,
    },
    /// Get password from the password store
    Get {
        #[arg(short, long)]
        get: bool,
    },
    /// Get all passwords from the password store
    GetAll {
        #[arg(short, long)]
        add: bool,
    },
    /// Remove password from the password store
    Remove {
        #[arg(short, long)]
        remove: bool,
    },
    /// Update password in the password store
    Update {
        #[arg(short, long)]
        update: bool,
    },
    /// Generate a new password
    Generate {
        #[arg(short, long)]
        generate: bool,
    },
}
