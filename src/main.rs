use std::{path::PathBuf, process};

use anyhow::Result;
use dotenv::dotenv;
use tracing::{debug, error};
use tracing_subscriber::{EnvFilter, fmt};

pub mod app;
pub use app::*;

pub mod cli;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_ok() {
        fmt().with_env_filter(EnvFilter::from_default_env()).init();
    }

    let matches = match cli::build_cli().try_get_matches() {
        Err(err) => {
            println!("{err}");
            return Ok(());
        }
        Ok(matches) => matches,
    };

    debug!("{:?}", matches);

    let query = matches.get_one::<PathBuf>("query")
        .expect("required")
        .clone();
    
    let connection_string: Vec<ConnectionString> = matches
        .get_many::<ConnectionString>("connection_string")
        .expect("required")
        .cloned()
        .collect();

    let app = App::new(connection_string, query).await?;

    let result = app.execute_query_from_file().await;

    match result {
        Ok(_) => Ok(()),

        Err(err) => {
            error!("{err}");
            eprintln!("{}", err.to_string());
            process::exit(1)
        }
    }
}
