use std::{path::PathBuf, process};

use anyhow::Result;
use dotenv::dotenv;
use tracing::{debug, error};
use tracing_subscriber::{EnvFilter, fmt};

pub mod app;
pub use app::*;

use crate::config::default_config_path;

pub mod cli;
pub mod config;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_ok() {
        fmt().with_env_filter(EnvFilter::from_default_env()).init();
    }

    if let Ok(matches) = cli::build_arguments(cli::CliOptions {
        query_required: false,
        connection_string_required: false,
    })
    .try_get_matches()
        && matches.get_flag("generate_config")
    {
        let config_path = matches
            .get_one::<PathBuf>("config")
            .expect("has default value")
            .clone();

        config::Config::generate_to_file(&config_path).await?;
        println!("Config file generated at: {}", config_path.display());
        return Ok(());
    }

    let (query, connection_strings) =
        match cli::build_arguments(cli::CliOptions::required())
            .try_get_matches()
        {
            Ok(matches) => {
                debug!("{:?}", matches);

                let query = matches
                    .get_one::<PathBuf>("query")
                    .expect("required")
                    .clone();

                let connection_strings: Vec<ConnectionString> = matches
                    .get_many::<ConnectionString>("connection_string")
                    .expect("required")
                    .cloned()
                    .collect();

                (query, connection_strings)
            }
            Err(_) => {
                let matches = cli::build_arguments(cli::CliOptions {
                    query_required: true,
                    connection_string_required: false,
                })
                .get_matches();

                debug!("{:?}", matches);

                let query = matches
                    .get_one::<PathBuf>("query")
                    .expect("required")
                    .clone();

                let config_path = matches
                    .get_one::<PathBuf>("config")
                    .expect("has default value")
                    .clone();

                if config_path == default_config_path()?
                    && !config_path.exists()
                {
                    cli::build_arguments(cli::CliOptions::required())
                        .get_matches();
                }

                debug!("Loading config from: {}", config_path.display());
                let cfg = config::Config::load_from_file(&config_path).await?;

                (query, cfg.connection_strings)
            }
        };

    let app = App::new(connection_strings, query).await?;

    let result = app.execute_query_from_file().await;

    match result {
        Ok(_) => Ok(()),

        Err(err) => {
            error!("{err}");
            eprintln!("{}", err);
            process::exit(1)
        }
    }
}
