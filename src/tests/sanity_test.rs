use super::utils::{
    build_cli, create_query_file, create_test_sqlite_db, parse_json_lines,
    run_cli,
};
use serde_json::json;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_query_single_database() {
    let setup_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            status TEXT NOT NULL
        );
        INSERT INTO users (id, name, status) VALUES (1, 'Alice', 'Active');
        INSERT INTO users (id, name, status) VALUES (2, 'Bob', 'Suspended');
        INSERT INTO users (id, name, status) VALUES (3, 'Charlie', 'Active');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM users WHERE status = 'Active' ORDER BY id;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![("test_db".to_string(), db_uri)];
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

#[test]
fn test_query_multiple_databases() {
    let setup_sql = r#"
        CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL
        );
        INSERT INTO products (id, name, price) VALUES (1, 'Widget', 10.99);
        INSERT INTO products (id, name, price) VALUES (2, 'Gadget', 25.50);
    "#;

    let (_temp_dir_1, db_uri_1) = create_test_sqlite_db(setup_sql);

    let setup_sql_2 = r#"
        CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL
        );
        INSERT INTO products (id, name, price) VALUES (1, 'Doohickey', 15.75);
        INSERT INTO products (id, name, price) VALUES (2, 'Thingamajig', 30.00);
    "#;

    let (_temp_dir_2, db_uri_2) = create_test_sqlite_db(setup_sql_2);

    let setup_sql_3 = r#"
        CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL
        );
        INSERT INTO products (id, name, price) VALUES (1, 'Gizmo', 20.00);
        INSERT INTO products (id, name, price) VALUES (2, 'Contraption', 18.25);
        INSERT INTO products (id, name, price) VALUES (3, 'Apparatus', 35.99);
    "#;

    let (_temp_dir_3, db_uri_3) = create_test_sqlite_db(setup_sql_3);

    let query = "SELECT * FROM products WHERE price > 15.0 ORDER BY price;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![
        ("db1".to_string(), db_uri_1),
        ("db2".to_string(), db_uri_2),
        ("db3".to_string(), db_uri_3),
    ];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    let db1_results: Vec<_> =
        results.iter().filter(|r| r["db_name"] == "db1").cloned().collect();
    let db2_results: Vec<_> =
        results.iter().filter(|r| r["db_name"] == "db2").cloned().collect();
    let db3_results: Vec<_> =
        results.iter().filter(|r| r["db_name"] == "db3").cloned().collect();

    let expected_db1 = vec![json!({
        "db_name": "db1",
        "id": 2,
        "name": "Gadget",
        "price": 25.5
    })];

    let expected_db2 = vec![
        json!({
            "db_name": "db2",
            "id": 1,
            "name": "Doohickey",
            "price": 15.75
        }),
        json!({
            "db_name": "db2",
            "id": 2,
            "name": "Thingamajig",
            "price": 30.0
        }),
    ];

    let expected_db3 = vec![
        json!({
            "db_name": "db3",
            "id": 2,
            "name": "Contraption",
            "price": 18.25
        }),
        json!({
            "db_name": "db3",
            "id": 1,
            "name": "Gizmo",
            "price": 20.0
        }),
        json!({
            "db_name": "db3",
            "id": 3,
            "name": "Apparatus",
            "price": 35.99
        }),
    ];

    assert_eq!(db1_results, expected_db1);
    assert_eq!(db2_results, expected_db2);
    assert_eq!(db3_results, expected_db3);
}

#[test]
fn test_empty_result_set() {
    let setup_sql = r#"
        CREATE TABLE items (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL
        );
        INSERT INTO items (id, name, category) VALUES (1, 'Item1', 'A');
        INSERT INTO items (id, name, category) VALUES (2, 'Item2', 'B');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM items WHERE category = 'C';";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![("test_db".to_string(), db_uri)];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);
    let expected: Vec<serde_json::Value> = vec![];

    assert_eq!(results, expected);
}

#[test]
fn test_query_with_aggregation() {
    let setup_sql = r#"
        CREATE TABLE orders (
            id INTEGER PRIMARY KEY,
            customer_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            status TEXT NOT NULL
        );
        INSERT INTO orders (id, customer_id, amount, status) VALUES (1, 100, 50.00, 'completed');
        INSERT INTO orders (id, customer_id, amount, status) VALUES (2, 100, 75.00, 'completed');
        INSERT INTO orders (id, customer_id, amount, status) VALUES (3, 200, 100.00, 'completed');
        INSERT INTO orders (id, customer_id, amount, status) VALUES (4, 200, 25.00, 'pending');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = r#"
        SELECT customer_id, COUNT(*) as order_count, SUM(amount) as total_amount
        FROM orders
        WHERE status = 'completed'
        GROUP BY customer_id
        ORDER BY customer_id;
    "#;
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![("test_db".to_string(), db_uri)];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    let expected = vec![
        json!({
            "db_name": "test_db",
            "customer_id": 100,
            "order_count": 2,
            "total_amount": 125.0
        }),
        json!({
            "db_name": "test_db",
            "customer_id": 200,
            "order_count": 1,
            "total_amount": 100.0
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_invalid_query_file() {
    let cli_path = build_cli();
    let non_existent_file =
        PathBuf::from("/tmp/non_existent_query_file_12345.sql");

    let output = Command::new(&cli_path)
        .arg("--query")
        .arg(non_existent_file)
        .arg("--connection-string")
        .arg("test,sqlite::memory:")
        .output()
        .expect("Failed to execute CLI");

    assert!(!output.status.success() || output.stdout.is_empty());
}

#[test]
fn test_query_nonexistent_table_returns_error() {
    let setup_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        );
        INSERT INTO users (id, name) VALUES (1, 'Alice');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM nonexistent_table;";
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings = vec![("test_db".to_string(), db_uri)];

    let result = run_cli(&cli_path, query_file.path(), &connection_strings);

    let error =
        result.expect_err("Expected error when querying nonexistent table");
    let error_msg = format!("{}", error);

    let expected_error = "error returned from database: (code: 1) no such table: nonexistent_table\n";
    assert_eq!(error_msg, expected_error);
}
