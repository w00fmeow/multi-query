use super::utils::{
    build_cli, create_query_file, create_test_postgres_db, parse_json_lines,
    run_cli,
};
use serde_json::json;
use std::path::PathBuf;

#[tokio::test]
async fn test_query_single_database() {
    let setup_sql = r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            status TEXT NOT NULL
        );
        INSERT INTO users (name, status) VALUES ('Alice', 'Active');
        INSERT INTO users (name, status) VALUES ('Bob', 'Suspended');
        INSERT INTO users (name, status) VALUES ('Charlie', 'Active');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM users WHERE status = 'Active' ORDER BY id;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    let expected = vec![
        json!({
            "db_name": "test_db",
            "id": 1,
            "name": "Alice",
            "status": "Active"
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "name": "Charlie",
            "status": "Active"
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_query_multiple_databases() {
    let setup_sql = r#"
        CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            price DECIMAL(10,2) NOT NULL
        );
        INSERT INTO products (name, price) VALUES ('Widget', 19.99);
        INSERT INTO products (name, price) VALUES ('Gadget', 29.99);
    "#;

    let pg_container1 = create_test_postgres_db(setup_sql).await;
    let pg_container2 = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM products ORDER BY id;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![
        ("db1".to_string(), pg_container1.uri.clone()),
        ("db2".to_string(), pg_container2.uri.clone()),
    ];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let mut results = parse_json_lines(&output);

    // Sort by db_name then id to handle async execution order
    results.sort_by(|a, b| {
        let db_a = a.get("db_name").and_then(|v| v.as_str()).unwrap_or("");
        let db_b = b.get("db_name").and_then(|v| v.as_str()).unwrap_or("");
        let id_a = a.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
        let id_b = b.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
        (db_a, id_a).cmp(&(db_b, id_b))
    });

    let expected = vec![
        json!({
            "db_name": "db1",
            "id": 1,
            "name": "Widget",
            "price": 19.99
        }),
        json!({
            "db_name": "db1",
            "id": 2,
            "name": "Gadget",
            "price": 29.99
        }),
        json!({
            "db_name": "db2",
            "id": 1,
            "name": "Widget",
            "price": 19.99
        }),
        json!({
            "db_name": "db2",
            "id": 2,
            "name": "Gadget",
            "price": 29.99
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_empty_result_set() {
    let setup_sql = r#"
        CREATE TABLE tasks (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        );
        INSERT INTO tasks (title, completed) VALUES ('Task 1', true);
        INSERT INTO tasks (title, completed) VALUES ('Task 2', true);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM tasks WHERE completed = false;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    assert_eq!(results, Vec::<serde_json::Value>::new());
}

#[tokio::test]
async fn test_query_nonexistent_table_returns_error() {
    let setup_sql = r#"
        CREATE TABLE real_table (
            id SERIAL PRIMARY KEY
        );
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM nonexistent_table;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let result = run_cli(&cli_path, query_file.path(), &connection_strings);

    assert!(result.is_err());
    let error_msg = result.unwrap_err();
    assert!(
        error_msg.contains("relation") || error_msg.contains("does not exist"),
        "Expected table not found error, got: {}",
        error_msg
    );
}

#[tokio::test]
async fn test_query_with_aggregation() {
    let setup_sql = r#"
        CREATE TABLE sales (
            id SERIAL PRIMARY KEY,
            product TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            price DECIMAL(10,2) NOT NULL
        );
        INSERT INTO sales (product, quantity, price) VALUES ('Widget', 5, 10.00);
        INSERT INTO sales (product, quantity, price) VALUES ('Widget', 3, 10.00);
        INSERT INTO sales (product, quantity, price) VALUES ('Gadget', 2, 15.00);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = r#"
        SELECT 
            product,
            SUM(quantity) as total_quantity,
            SUM(quantity * price) as total_revenue
        FROM sales
        GROUP BY product
        ORDER BY product;
    "#;
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    let expected = vec![
        json!({
            "db_name": "test_db",
            "product": "Gadget",
            "total_quantity": 2,
            "total_revenue": 30.0
        }),
        json!({
            "db_name": "test_db",
            "product": "Widget",
            "total_quantity": 8,
            "total_revenue": 80.0
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_invalid_query_file() {
    let pg_container = create_test_postgres_db("").await;

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let result = run_cli(
        &cli_path,
        &PathBuf::from("/nonexistent/query/file.sql"),
        &connection_strings,
    );

    assert!(result.is_err());
}
