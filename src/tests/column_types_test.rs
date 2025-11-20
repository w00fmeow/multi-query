use super::utils::{
    build_cli, create_query_file, create_test_sqlite_db, parse_json_lines,
    run_cli,
};
use serde_json::json;

#[test]
fn test_integer_types() {
    let setup_sql = r#"
        CREATE TABLE integers (
            id INTEGER PRIMARY KEY,
            tiny_int TINYINT,
            small_int SMALLINT,
            medium_int MEDIUMINT,
            big_int BIGINT
        );
        INSERT INTO integers (id, tiny_int, small_int, medium_int, big_int) 
        VALUES (1, 127, 32767, 8388607, 9223372036854775807);
        INSERT INTO integers (id, tiny_int, small_int, medium_int, big_int) 
        VALUES (2, -128, -32768, -8388608, -9223372036854775808);
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM integers ORDER BY id;";
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
            "tiny_int": 127,
            "small_int": 32767,
            "medium_int": 8388607,
            "big_int": 9223372036854775807_i64
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "tiny_int": -128,
            "small_int": -32768,
            "medium_int": -8388608,
            "big_int": -9223372036854775808_i64
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_float_types() {
    let setup_sql = r#"
        CREATE TABLE floats (
            id INTEGER PRIMARY KEY,
            float_val REAL,
            double_val DOUBLE,
            decimal_val DECIMAL(10,2)
        );
        INSERT INTO floats (id, float_val, double_val, decimal_val) 
        VALUES (1, 3.14159, 2.718281828, 123.45);
        INSERT INTO floats (id, float_val, double_val, decimal_val) 
        VALUES (2, -99.99, 0.0001, -456.78);
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM floats ORDER BY id;";
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
            "float_val": 3.14159,
            "double_val": 2.718281828,
            "decimal_val": 123.45
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "float_val": -99.99,
            "double_val": 0.0001,
            "decimal_val": -456.78
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_string_types() {
    let setup_sql = r#"
        CREATE TABLE strings (
            id INTEGER PRIMARY KEY,
            varchar_val VARCHAR(100),
            text_val TEXT,
            char_val CHAR(10),
            blob_val BLOB
        );
        INSERT INTO strings (id, varchar_val, text_val, char_val) 
        VALUES (1, 'Hello World', 'This is a long text', 'FIXED');
        INSERT INTO strings (id, varchar_val, text_val, char_val) 
        VALUES (2, 'Special: àéîöü', 'Multi
Line
Text', 'A');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query =
        "SELECT id, varchar_val, text_val, char_val FROM strings ORDER BY id;";
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
            "varchar_val": "Hello World",
            "text_val": "This is a long text",
            "char_val": "FIXED"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "varchar_val": "Special: àéîöü",
            "text_val": "Multi\nLine\nText",
            "char_val": "A"
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_boolean_type() {
    let setup_sql = r#"
        CREATE TABLE booleans (
            id INTEGER PRIMARY KEY,
            is_active BOOLEAN,
            is_verified BOOLEAN,
            is_deleted BOOLEAN
        );
        INSERT INTO booleans (id, is_active, is_verified, is_deleted) 
        VALUES (1, 1, 0, 0);
        INSERT INTO booleans (id, is_active, is_verified, is_deleted) 
        VALUES (2, 0, 1, 1);
        INSERT INTO booleans (id, is_active, is_verified, is_deleted) 
        VALUES (3, 1, 1, 0);
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM booleans ORDER BY id;";
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
            "is_active": 1,
            "is_verified": 0,
            "is_deleted": 0
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "is_active": 0,
            "is_verified": 1,
            "is_deleted": 1
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "is_active": 1,
            "is_verified": 1,
            "is_deleted": 0
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_date_and_time_types() {
    let setup_sql = r#"
        CREATE TABLE dates (
            id INTEGER PRIMARY KEY,
            date_val DATE,
            time_val TIME,
            datetime_val DATETIME,
            timestamp_val TIMESTAMP
        );
        INSERT INTO dates (id, date_val, time_val, datetime_val, timestamp_val) 
        VALUES (1, '2024-01-15', '14:30:00', '2024-01-15 14:30:00', '2024-01-15 14:30:00');
        INSERT INTO dates (id, date_val, time_val, datetime_val, timestamp_val) 
        VALUES (2, '2023-12-31', '23:59:59', '2023-12-31 23:59:59', '2023-12-31 23:59:59');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM dates ORDER BY id;";
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
            "date_val": "2024-01-15",
            "time_val": "14:30:00",
            "datetime_val": "2024-01-15 14:30:00",
            "timestamp_val": "2024-01-15 14:30:00"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "date_val": "2023-12-31",
            "time_val": "23:59:59",
            "datetime_val": "2023-12-31 23:59:59",
            "timestamp_val": "2023-12-31 23:59:59"
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_null_values() {
    let setup_sql = r#"
        CREATE TABLE nulls (
            id INTEGER PRIMARY KEY,
            nullable_int INTEGER,
            nullable_text TEXT,
            nullable_float REAL
        );
        INSERT INTO nulls (id, nullable_int, nullable_text, nullable_float) 
        VALUES (1, 42, 'text', 3.14);
        INSERT INTO nulls (id, nullable_int, nullable_text, nullable_float) 
        VALUES (2, NULL, NULL, NULL);
        INSERT INTO nulls (id, nullable_int, nullable_text, nullable_float) 
        VALUES (3, 0, '', 0.0);
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM nulls ORDER BY id;";
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
            "nullable_int": 42,
            "nullable_text": "text",
            "nullable_float": 3.14
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "nullable_int": null,
            "nullable_text": null,
            "nullable_float": null
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "nullable_int": 0,
            "nullable_text": "",
            "nullable_float": 0.0
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_json_type() {
    let setup_sql = r#"
        CREATE TABLE json_data (
            id INTEGER PRIMARY KEY,
            json_val TEXT
        );
        INSERT INTO json_data (id, json_val) 
        VALUES (1, '  {"name": "John", "age": 30, "active": true}  ');
        INSERT INTO json_data (id, json_val) 
        VALUES (2, '{"items":  ["apple", "banana",  "cherry"], "count":  3}');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = r#"
        SELECT 
            id,
            json(json_val) as data
        FROM json_data 
        ORDER BY id;
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
            "id": 1,
            "data": "{\"name\":\"John\",\"age\":30,\"active\":true}"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "data": "{\"items\":[\"apple\",\"banana\",\"cherry\"],\"count\":3}"
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_json_function() {
    let setup_sql = r#"
        CREATE TABLE json_data (
            id INTEGER PRIMARY KEY,
            json_val TEXT
        );
        INSERT INTO json_data (id, json_val) 
        VALUES (1, '  {"name":  "John",   "age": 30}  ');
        INSERT INTO json_data (id, json_val) 
        VALUES (2, '{"items":["apple","banana"]}');
        INSERT INTO json_data (id, json_val) 
        VALUES (3, 'invalid json');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = r#"
        SELECT 
            id,
            json(json_val) as data
        FROM json_data 
        WHERE json_valid(json_val)
        ORDER BY id;
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
            "id": 1,
            "data": "{\"name\":\"John\",\"age\":30}"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "data": "{\"items\":[\"apple\",\"banana\"]}"
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_enum_types() {
    let setup_sql = r#"
        CREATE TABLE products (
            id INTEGER PRIMARY KEY,
            name TEXT,
            status TEXT CHECK(status IN ('active', 'inactive', 'pending')),
            category TEXT CHECK(category IN ('electronics', 'clothing', 'food'))
        );
        INSERT INTO products (id, name, status, category) 
        VALUES (1, 'Laptop', 'active', 'electronics');
        INSERT INTO products (id, name, status, category) 
        VALUES (2, 'Shirt', 'inactive', 'clothing');
        INSERT INTO products (id, name, status, category) 
        VALUES (3, 'Apple', 'pending', 'food');
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM products ORDER BY id;";
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
            "name": "Laptop",
            "status": "active",
            "category": "electronics"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "name": "Shirt",
            "status": "inactive",
            "category": "clothing"
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "name": "Apple",
            "status": "pending",
            "category": "food"
        }),
    ];

    assert_eq!(results, expected);
}

#[test]
fn test_null_dates() {
    let setup_sql = r#"
        CREATE TABLE events (
            id INTEGER PRIMARY KEY,
            event_name TEXT,
            start_date DATE,
            end_date DATE
        );
        INSERT INTO events (id, event_name, start_date, end_date) 
        VALUES (1, 'Conference', '2024-06-15', '2024-06-17');
        INSERT INTO events (id, event_name, start_date, end_date) 
        VALUES (2, 'Ongoing Event', '2024-01-01', NULL);
        INSERT INTO events (id, event_name, start_date, end_date) 
        VALUES (3, 'TBD Event', NULL, NULL);
    "#;

    let (_temp_dir, db_uri) = create_test_sqlite_db(setup_sql);

    let query = "SELECT * FROM events ORDER BY id;";
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
            "event_name": "Conference",
            "start_date": "2024-06-15",
            "end_date": "2024-06-17"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "event_name": "Ongoing Event",
            "start_date": "2024-01-01",
            "end_date": null
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "event_name": "TBD Event",
            "start_date": null,
            "end_date": null
        }),
    ];

    assert_eq!(results, expected);
}
