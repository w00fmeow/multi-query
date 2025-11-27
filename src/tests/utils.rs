use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::NamedTempFile;
use testcontainers::core::IntoContainerPort;
use testcontainers::{ContainerAsync, runners::AsyncRunner};
use testcontainers_modules::postgres::Postgres;

pub fn create_query_file(query: &str) -> NamedTempFile {
    let mut file =
        NamedTempFile::new().expect("Failed to create temp query file");
    file.write_all(query.as_bytes()).expect("Failed to write query");
    file.flush().expect("Failed to flush");
    file
}

pub fn build_cli() -> PathBuf {
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to build CLI");

    if !output.status.success() {
        panic!("Failed to build: {}", String::from_utf8_lossy(&output.stderr));
    }

    PathBuf::from("target/release/multi-query")
}

pub fn run_cli(
    cli_path: &PathBuf,
    query_file: &Path,
    connection_strings: &[(String, String)],
) -> Result<String, String> {
    let mut cmd = Command::new(cli_path);
    cmd.arg("--query").arg(query_file);

    for (name, uri) in connection_strings {
        cmd.arg("--connection-string").arg(format!("{},{}", name, uri));
    }

    let output = cmd.output().expect("Failed to execute CLI");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8(output.stdout).expect("Invalid UTF-8 output"))
}

pub fn parse_json_lines(output: &str) -> Vec<Value> {
    output
        .lines()
        .filter(|line| !line.is_empty() && line.trim().starts_with('{'))
        .map(|line| serde_json::from_str(line).expect("Failed to parse JSON"))
        .collect()
}

pub struct PostgresContainer {
    _container: ContainerAsync<Postgres>,
    pub uri: String,
}

pub async fn create_test_postgres_db(setup_sql: &str) -> PostgresContainer {
    let postgres = if setup_sql.is_empty() {
        Postgres::default()
    } else {
        Postgres::default().with_init_sql(setup_sql.as_bytes().to_vec())
    };

    let container =
        postgres.start().await.expect("Failed to start PostgreSQL container");

    let host = container.get_host().await.expect("Failed to get host");
    let port = container
        .get_host_port_ipv4(5432.tcp())
        .await
        .expect("Failed to get port");
    let uri =
        format!("postgresql://postgres:postgres@{}:{}/postgres", host, port);

    PostgresContainer { _container: container, uri }
}
