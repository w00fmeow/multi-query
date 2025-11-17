use serde_json::Value;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::{NamedTempFile, TempDir};
use uuid::Uuid;

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

fn random_db_name() -> String {
    format!("db_{}", Uuid::new_v4().simple())
}

pub fn create_test_sqlite_db(setup_sql: &str) -> (TempDir, String) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let db_name = random_db_name();
    let db_path = temp_dir.path().join(format!("{}.db", db_name));
    let uri = format!("sqlite://{}", db_path.display());

    let mut child = Command::new("sqlite3")
        .arg(&db_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn sqlite3");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(setup_sql.as_bytes())
            .expect("Failed to write to sqlite3 stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait for sqlite3");
    if !output.status.success() {
        panic!(
            "Failed to setup database: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    (temp_dir, uri)
}
