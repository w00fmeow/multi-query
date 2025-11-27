use super::utils::{build_cli, create_test_postgres_db, parse_json_lines};
use crate::ConnectionString;
use crate::config::Config;
use serde_json::json;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_generate_config_with_default_path() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    let output = Command::new(&cli_path)
        .args(["--generate-config", "--config", config_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Config file generated at:"));
    assert!(config_path.exists());

    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();

    let expected = Config {
        connection_strings: vec![ConnectionString {
            name: "example_db".to_string(),
            uri: "postgresql://user:password@localhost:5432/database"
                .to_string(),
        }],
    };

    assert_eq!(config, expected);
}

#[test]
fn test_generate_config_creates_nested_directories() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let nested_path =
        temp_dir.path().join("nested").join("dirs").join("config.json");

    let output = Command::new(&cli_path)
        .args(["--generate-config", "--config", nested_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    assert!(nested_path.exists());
}

#[test]
fn test_generate_config_overwrites_existing() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");

    std::fs::write(&config_path, "old content").unwrap();

    let output = Command::new(&cli_path)
        .args(["--generate-config", "--config", config_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());

    let content = std::fs::read_to_string(&config_path).unwrap();
    let config: Config = serde_json::from_str(&content).unwrap();

    let expected = Config {
        connection_strings: vec![ConnectionString {
            name: "example_db".to_string(),
            uri: "postgresql://user:password@localhost:5432/database"
                .to_string(),
        }],
    };

    assert_eq!(config, expected);
}

#[tokio::test]
async fn test_load_valid_config_file() {
    let pg_container = create_test_postgres_db("").await;
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    let query_file = temp_dir.path().join("query.sql");

    let config_content = format!(
        r#"{{
        "connection_strings": [
            {{"name": "test_db", "uri": "{}"}}
        ]
    }}"#,
        pg_container.uri
    );

    std::fs::write(&config_path, config_content).unwrap();
    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            config_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let results = parse_json_lines(&stdout);

    let expected = vec![json!({
        "?column?": 1,
        "db_name": "test_db"
    })];

    assert_eq!(results, expected);
}

#[test]
fn test_empty_connection_strings_fails() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    let query_file = temp_dir.path().join("query.sql");

    let config_content = r#"{
        "connection_strings": []
    }"#;

    std::fs::write(&config_path, config_content).unwrap();
    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            config_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("connection_strings cannot be empty"));
}

#[test]
fn test_nonexistent_config_file_fails() {
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
    assert!(stderr.contains("Failed to read"));
}

#[test]
fn test_invalid_json_config_fails() {
    let cli_path = build_cli();
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.json");
    let query_file = temp_dir.path().join("query.sql");

    std::fs::write(&config_path, "not valid json").unwrap();
    std::fs::write(&query_file, "SELECT 1").unwrap();

    let output = Command::new(&cli_path)
        .args([
            "--query",
            query_file.to_str().unwrap(),
            "--config",
            config_path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to parse"));
}
