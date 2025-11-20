use std::process;

use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use tracing::{debug, error};
use tracing_subscriber::{EnvFilter, fmt};

pub mod app;
pub use app::*;

use crate::cli::Arguments;

pub mod cli;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    if std::env::var("RUST_LOG").is_ok() {
        fmt().with_env_filter(EnvFilter::from_default_env()).init();
    }

    let args = match Arguments::try_parse() {
        Err(err) => {
            println!("{err}");
            return Ok(());
        }
        Ok(args) => args,
    };

    debug!("{:?}", args);

    let app = App::new(args.connection_string, args.query).await?;

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
