use clap::{command, value_parser, Arg, ArgAction, Command};

use crate::ConnectionString;

pub fn build_cli() -> Command {
    command!() // Uses cargo metadata for name, version, author, about
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("FILE")
                .help("Path to SQL query file to execute across all databases")
                .required(true)
                .value_parser(value_parser!(std::path::PathBuf))
        )
        .arg(
            Arg::new("connection_string")
                .short('c')
                .long("connection-string")
                .value_name("NAME,URI")
                .help("Database connection string in format: <name>,<uri>\n\nExamples:\n  prod-db,postgresql://user:pass@localhost/dbname\n\nCan be specified multiple times to query multiple databases")
                .required(true)
                .action(ArgAction::Append)
                .value_parser(value_parser!(ConnectionString))
        )
}
