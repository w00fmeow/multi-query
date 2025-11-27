use clap::{Arg, ArgAction, Command, command, value_parser};

use crate::ConnectionString;

pub struct CliOptions {
    pub query_required: bool,
    pub connection_string_required: bool,
}

impl CliOptions {
    pub fn required() -> Self {
        Self { query_required: true, connection_string_required: true }
    }
}

pub fn build_arguments(options: CliOptions) -> Command {
    let home = dirs::home_dir().expect("Could not determine home directory");
    let app_name = env!("CARGO_PKG_NAME");
    let config_dir = format!(".{}", app_name);
    let config_filename = "config.json";

    let default_config_path: &'static str = Box::leak(
        home.join(&config_dir)
            .join(config_filename)
            .display()
            .to_string()
            .into_boxed_str(),
    );

    let config_help: &'static str = Box::leak(
        format!(
            "Path to config file [default: ~/{}/{}]",
            config_dir, config_filename
        )
        .into_boxed_str(),
    );

    command!()
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .value_name("FILE")
                .help("Path to SQL query file to execute across all databases")
                .required(options.query_required)
                .value_parser(value_parser!(std::path::PathBuf))
        )
        .arg(
            Arg::new("connection_string")
                .short('c')
                .long("connection-string")
                .value_name("NAME,URI")
                .help("Database connection string in format: <name>,<uri>\n\nExamples:\n  prod-db,postgresql://user:pass@localhost/dbname\n\nCan be specified multiple times to query multiple databases")
                .required(options.connection_string_required)
                .action(ArgAction::Append)
                .value_parser(value_parser!(ConnectionString))
        )
        .arg(
            Arg::new("config")
                    .long("config")
                    .value_name("FILE")
                    .help(config_help)
                    .default_value(default_config_path)
                    .hide_default_value(true)
                    .value_parser(value_parser!(std::path::PathBuf))
        )
        .arg(
            Arg::new("generate_config")
                .long("generate-config")
                .help("Generate a default config file at the config path")
                .action(ArgAction::SetTrue)
        )
}
