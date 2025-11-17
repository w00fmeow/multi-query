use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::ConnectionString;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    /// Path to SQL query file to execute across all databases
    #[arg(short, long, value_name = "FILE")]
    pub query: PathBuf,

    /// Database connection string in format: <name>,<uri>
    ///
    /// Examples:
    ///   prod-db,postgresql://user:pass@localhost/dbname
    ///
    /// Can be specified multiple times to query multiple databases
    #[arg(short, long, value_name = "NAME,URI", action = ArgAction::Append)]
    pub connection_string: Vec<ConnectionString>,
}
