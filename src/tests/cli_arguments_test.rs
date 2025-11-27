use super::utils::{build_cli, create_test_postgres_db};
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_no_arguments() {
    let cli_path = build_cli();

    let output =
        Command::new(&cli_path).output().expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    let expected = "error: the following required arguments were not provided:
  --query <FILE>

Usage: multi-query --query <FILE>

For more information, try '--help'.
";

    assert_eq!(stderr, expected);
}

#[test]
fn test_only_query_argument_with_nonexistent_default_config() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");
    let nonexistent_config = temp_dir.path().join("nonexistent_default.json");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            nonexistent_config.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Failed to read config file"));
}

#[test]
fn test_only_connection_string_argument() {
    let cli_path = build_cli();

    let output = Command::new(&cli_path)
        .args(["--connection-string", "test,sqlite://:memory:"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains(
        "error: the following required arguments were not provided:"
    ));
    assert!(stderr.contains("--query <FILE>"));
}

#[tokio::test]
async fn test_query_and_connection_string_success() {
    let pg_container = create_test_postgres_db("").await;
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--connection-string",
            &format!("test,{}", pg_container.uri),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_query_with_nonexistent_config() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");
    let nonexistent_config = temp_dir.path().join("nonexistent.json");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            nonexistent_config.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to read config file"));
}

#[tokio::test]
async fn test_query_with_valid_config() {
    let pg_container = create_test_postgres_db("").await;
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");
    let config_file = temp_dir.path().join("config.json");

    std::fs::write(&query_file, "SELECT 1").unwrap();
    std::fs::write(
        &config_file,
        &format!(
            r#"{{"connection_strings":[{{"name":"test","uri":"{}"}}]}}"#,
            pg_container.uri
        ),
    )
    .unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            config_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[tokio::test]
async fn test_multiple_connection_strings() {
    let pg_container1 = create_test_postgres_db("").await;
    let pg_container2 = create_test_postgres_db("").await;
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--connection-string",
            &format!("db1,{}", pg_container1.uri),
            "--connection-string",
            &format!("db2,{}", pg_container2.uri),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("db1"));
    assert!(stdout.contains("db2"));
}

#[test]
fn test_generate_config_without_other_args() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.json");

    let output = Command::new(&cli_path)
        .args(["--generate-config", "--config", config_file.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Config file generated at:"));
    assert!(config_file.exists());
}

#[test]
fn test_help_flag() {
    let cli_path = build_cli();

    let output = Command::new(&cli_path)
        .args(["--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--query"));
    assert!(stdout.contains("--connection-string"));
    assert!(stdout.contains("--config"));
    assert!(stdout.contains("--generate-config"));
}

#[test]
fn test_invalid_connection_string_format() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--connection-string",
            "invalid_format_without_comma",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected format"));
}

#[tokio::test]
async fn test_short_flags() {
    let pg_container = create_test_postgres_db("").await;
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let query_file = temp_dir.path().join("query.sql");

    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "-q",
            query_file.to_str().unwrap(),
            "-c",
            &format!("test,{}", pg_container.uri),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}
