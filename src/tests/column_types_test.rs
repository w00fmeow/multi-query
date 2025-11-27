use super::utils::{
    build_cli, create_query_file, create_test_postgres_db, parse_json_lines,
    run_cli,
};
use serde_json::json;

#[tokio::test]
async fn test_integer_types() {
    let setup_sql = r#"
        CREATE TABLE integers (
            id SERIAL PRIMARY KEY,
            tiny_int SMALLINT,
            small_int SMALLINT,
            medium_int INTEGER,
            big_int BIGINT
        );
        INSERT INTO integers (tiny_int, small_int, medium_int, big_int) 
        VALUES (127, 32767, 8388607, 9223372036854775807);
        INSERT INTO integers (tiny_int, small_int, medium_int, big_int) 
        VALUES (-128, -32768, -8388608, -9223372036854775808);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM integers ORDER BY id;";
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

#[tokio::test]
async fn test_float_types() {
    let setup_sql = r#"
        CREATE TABLE floats (
            id SERIAL PRIMARY KEY,
            float_val DOUBLE PRECISION,
            double_val DOUBLE PRECISION,
            decimal_val DECIMAL(10,2)
        );
        INSERT INTO floats (float_val, double_val, decimal_val) 
        VALUES (3.14159, 2.718281828, 123.45);
        INSERT INTO floats (float_val, double_val, decimal_val) 
        VALUES (-99.99, 0.0001, -456.78);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM floats ORDER BY id;";
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

#[tokio::test]
async fn test_string_types() {
    let setup_sql = r#"
        CREATE TABLE strings (
            id SERIAL PRIMARY KEY,
            varchar_val VARCHAR(100),
            text_val TEXT,
            char_val CHAR(10)
        );
        INSERT INTO strings (varchar_val, text_val, char_val) 
        VALUES ('Hello World', 'This is a long text', 'FIXED');
        INSERT INTO strings (varchar_val, text_val, char_val) 
        VALUES ('Special: àéîöü', E'Multi\nLine\nText', 'A');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query =
        "SELECT id, varchar_val, text_val, char_val FROM strings ORDER BY id;";
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
            "varchar_val": "Hello World",
            "text_val": "This is a long text",
            "char_val": "FIXED     "
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "varchar_val": "Special: àéîöü",
            "text_val": "Multi\nLine\nText",
            "char_val": "A         "
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_boolean_type() {
    let setup_sql = r#"
        CREATE TABLE booleans (
            id SERIAL PRIMARY KEY,
            is_active BOOLEAN,
            is_verified BOOLEAN,
            is_deleted BOOLEAN
        );
        INSERT INTO booleans (is_active, is_verified, is_deleted) 
        VALUES (true, false, false);
        INSERT INTO booleans (is_active, is_verified, is_deleted) 
        VALUES (false, true, true);
        INSERT INTO booleans (is_active, is_verified, is_deleted) 
        VALUES (true, true, false);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM booleans ORDER BY id;";
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
            "is_active": true,
            "is_verified": false,
            "is_deleted": false
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "is_active": false,
            "is_verified": true,
            "is_deleted": true
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "is_active": true,
            "is_verified": true,
            "is_deleted": false
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_date_and_time_types() {
    let setup_sql = r#"
        CREATE TABLE dates (
            id SERIAL PRIMARY KEY,
            date_val DATE,
            time_val TIME,
            datetime_val TIMESTAMP,
            timestamp_val TIMESTAMPTZ
        );
        INSERT INTO dates (date_val, time_val, datetime_val, timestamp_val) 
        VALUES ('2024-01-15', '14:30:00', '2024-01-15 14:30:00', '2024-01-15 14:30:00+00');
        INSERT INTO dates (date_val, time_val, datetime_val, timestamp_val) 
        VALUES ('2023-12-31', '23:59:59', '2023-12-31 23:59:59', '2023-12-31 23:59:59+00');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM dates ORDER BY id;";
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
            "date_val": "2024-01-15",
            "time_val": "14:30:00",
            "datetime_val": "2024-01-15T14:30:00",
            "timestamp_val": "2024-01-15T14:30:00Z"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "date_val": "2023-12-31",
            "time_val": "23:59:59",
            "datetime_val": "2023-12-31T23:59:59",
            "timestamp_val": "2023-12-31T23:59:59Z"
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_null_values() {
    let setup_sql = r#"
        CREATE TABLE nulls (
            id SERIAL PRIMARY KEY,
            nullable_int INTEGER,
            nullable_text TEXT,
            nullable_float DOUBLE PRECISION
        );
        INSERT INTO nulls (nullable_int, nullable_text, nullable_float) 
        VALUES (42, 'text', 3.14);
        INSERT INTO nulls (nullable_int, nullable_text, nullable_float) 
        VALUES (NULL, NULL, NULL);
        INSERT INTO nulls (nullable_int, nullable_text, nullable_float) 
        VALUES (0, '', 0.0);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM nulls ORDER BY id;";
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

#[tokio::test]
async fn test_json_type() {
    let setup_sql = r#"
        CREATE TABLE json_data (
            id SERIAL PRIMARY KEY,
            json_val JSONB
        );
        INSERT INTO json_data (json_val) 
        VALUES ('{"name": "John", "age": 30, "active": true}');
        INSERT INTO json_data (json_val) 
        VALUES ('{"items": ["apple", "banana", "cherry"], "count": 3}');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT id, json_val FROM json_data ORDER BY id;";
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
            "json_val": {
                "name": "John",
                "age": 30,
                "active": true
            }
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "json_val": {
                "items": ["apple", "banana", "cherry"],
                "count": 3
            }
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_json_function() {
    let setup_sql = r#"
        CREATE TABLE json_data (
            id SERIAL PRIMARY KEY,
            json_val TEXT
        );
        INSERT INTO json_data (json_val) 
        VALUES ('{"name": "John", "age": 30}');
        INSERT INTO json_data (json_val) 
        VALUES ('{"items":["apple","banana"]}');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = r#"
        SELECT 
            id,
            json_val::jsonb as data
        FROM json_data 
        ORDER BY id;
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
            "id": 1,
            "data": {
                "name": "John",
                "age": 30
            }
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "data": {
                "items": ["apple", "banana"]
            }
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_enum_types() {
    let setup_sql = r#"
        CREATE TYPE product_status AS ENUM ('active', 'inactive', 'pending');
        CREATE TYPE product_category AS ENUM ('electronics', 'clothing', 'food');
        
        CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name TEXT,
            status product_status,
            category product_category
        );
        INSERT INTO products (name, status, category) 
        VALUES ('Laptop', 'active', 'electronics');
        INSERT INTO products (name, status, category) 
        VALUES ('Shirt', 'inactive', 'clothing');
        INSERT INTO products (name, status, category) 
        VALUES ('Apple', 'pending', 'food');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM products ORDER BY id;";
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

#[tokio::test]
async fn test_null_dates() {
    let setup_sql = r#"
        CREATE TABLE events (
            id SERIAL PRIMARY KEY,
            event_name TEXT,
            start_date DATE,
            end_date DATE
        );
        INSERT INTO events (event_name, start_date, end_date) 
        VALUES ('Conference', '2024-06-15', '2024-06-17');
        INSERT INTO events (event_name, start_date, end_date) 
        VALUES ('Ongoing Event', '2024-01-01', NULL);
        INSERT INTO events (event_name, start_date, end_date) 
        VALUES ('TBD Event', NULL, NULL);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM events ORDER BY id;";
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

#[tokio::test]
async fn test_array_types() {
    let setup_sql = r#"
        CREATE TABLE test_arrays (
            id SERIAL PRIMARY KEY,
            text_array TEXT[],
            int_array INTEGER[],
            bool_array BOOLEAN[]
        );
        INSERT INTO test_arrays (text_array, int_array, bool_array) 
        VALUES 
            (ARRAY['apple', 'banana', 'cherry'], ARRAY[1, 2, 3], ARRAY[true, false, true]),
            (ARRAY['hello', 'world'], ARRAY[10, 20], ARRAY[false, false]);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM test_arrays ORDER BY id;";
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
            "text_array": ["apple", "banana", "cherry"],
            "int_array": [1, 2, 3],
            "bool_array": [true, false, true]
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "text_array": ["hello", "world"],
            "int_array": [10, 20],
            "bool_array": [false, false]
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_custom_enum_types() {
    let setup_sql = r#"
        CREATE TYPE mood AS ENUM ('happy', 'sad', 'neutral');
        CREATE TABLE test_enums (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            current_mood mood NOT NULL
        );
        INSERT INTO test_enums (name, current_mood) 
        VALUES 
            ('Alice', 'happy'),
            ('Bob', 'sad'),
            ('Charlie', 'neutral');
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM test_enums ORDER BY id;";
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
            "current_mood": "happy"
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "name": "Bob",
            "current_mood": "sad"
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "name": "Charlie",
            "current_mood": "neutral"
        }),
    ];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_array_agg_function() {
    let setup_sql = r#"
        CREATE TABLE organizations (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id INTEGER
        );
        INSERT INTO organizations (name, parent_id) VALUES 
            ('Parent Org', NULL),
            ('Child 1', 1),
            ('Child 2', 1),
            ('Child 3', 1);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = r#"
        SELECT 
            parent.id,
            parent.name as parent_name,
            ARRAY_AGG(child.name ORDER BY child.id) as children
        FROM organizations parent
        LEFT JOIN organizations child ON child.parent_id = parent.id
        WHERE parent.parent_id IS NULL
        GROUP BY parent.id, parent.name;
    "#;
    let query_file = create_query_file(query);

    let cli_path = build_cli();
    let connection_strings =
        vec![("test_db".to_string(), pg_container.uri.clone())];
    let output = run_cli(&cli_path, query_file.path(), &connection_strings)
        .expect("CLI execution failed");

    let results = parse_json_lines(&output);

    let expected = vec![json!({
        "db_name": "test_db",
        "id": 1,
        "parent_name": "Parent Org",
        "children": ["Child 1", "Child 2", "Child 3"]
    })];

    assert_eq!(results, expected);
}

#[tokio::test]
async fn test_nullable_arrays() {
    let setup_sql = r#"
        CREATE TABLE test_nullable_arrays (
            id SERIAL PRIMARY KEY,
            optional_array TEXT[]
        );
        INSERT INTO test_nullable_arrays (optional_array) 
        VALUES 
            (ARRAY['item1', 'item2']),
            (NULL),
            (ARRAY[]::TEXT[]);
    "#;

    let pg_container = create_test_postgres_db(setup_sql).await;

    let query = "SELECT * FROM test_nullable_arrays ORDER BY id;";
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
            "optional_array": ["item1", "item2"]
        }),
        json!({
            "db_name": "test_db",
            "id": 2,
            "optional_array": null
        }),
        json!({
            "db_name": "test_db",
            "id": 3,
            "optional_array": []
        }),
    ];

    assert_eq!(results, expected);
}
