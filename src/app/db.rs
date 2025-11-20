use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use serde_json::Value;
use serde_json::json;
use serde_json::to_string;
use tokio_stream::StreamExt;

use sqlx::postgres::{PgPool, PgRow};
use sqlx::sqlite::{SqlitePool, SqliteRow};
use sqlx::{Column, Row, ValueRef};

use crate::ConnectionString;

enum DbPool {
    Postgres(PgPool),
    Sqlite(SqlitePool),
}

pub struct Db {
    pub name: String,
    db: DbPool,
}

macro_rules! row_to_json_impl {
    ($self:expr, $row:expr, $db_type:ty) => {{
        let mut json_obj = serde_json::Map::new();
        let mut key_count: HashMap<String, usize> = HashMap::new();

        json_obj
            .insert("db_name".to_string(), Value::String($self.name.clone()));

        for col in $row.columns() {
            let name = col.name();
            let count = key_count
                .entry(name.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(0);
            let key = if *count == 0 {
                name
            } else {
                &format!("{}_{}", name, count)
            };

            let json_value: Value = if let Ok(Some(v)) =
                $row.try_get::<Option<i32>, _>(name)
            {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<i64>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<f64>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<bool>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<String>, _>(name)
            {
                Value::String(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<&str>, _>(name) {
                Value::String(v.to_string())
            } else if let Ok(Some(v)) = $row.try_get::<Option<NaiveDateTime>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<NaiveDate>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<NaiveTime>, _>(name) {
                json!(v)
            } else if let Ok(Some(v)) = $row.try_get::<Option<DateTime<Utc>>, _>(name) {
                json!(v)
            } else if let Ok(None) = $row.try_get::<Option<i32>, _>(name) {
                Value::Null
            } else if let Ok(None) = $row.try_get::<Option<i64>, _>(name) {
                Value::Null
            } else if let Ok(None) = $row.try_get::<Option<f64>, _>(name) {
                Value::Null
            } else if let Ok(None) = $row.try_get::<Option<bool>, _>(name) {
                Value::Null
            } else if let Ok(None) = $row.try_get::<Option<String>, _>(name) {
                Value::Null
            } else {
                // For enums and other types, try to decode as string using database-specific Decode
                match $row.try_get_raw(name) {
                    Ok(raw) if !raw.is_null() => {
                        match <String as sqlx::Decode<$db_type>>::decode(raw) {
                            Ok(s) => Value::String(s),
                            Err(_) => Value::Null
                        }
                    }
                    _ => Value::Null
                }
            };

            json_obj.insert(key.to_string(), json_value);
        }

        json_obj
    }};
}

impl Db {
    pub async fn new(
        ConnectionString { name, uri }: ConnectionString,
    ) -> Result<Self> {
        let db = if uri.starts_with("postgres://")
            || uri.starts_with("postgresql://")
        {
            DbPool::Postgres(PgPool::connect(&uri).await?)
        } else if uri.starts_with("sqlite://") {
            DbPool::Sqlite(SqlitePool::connect(&uri).await?)
        } else {
            anyhow::bail!("Unsupported database type");
        };

        Ok(Self { db, name })
    }

    fn row_to_json_pg(&self, row: PgRow) -> serde_json::Map<String, Value> {
        row_to_json_impl!(self, row, sqlx::Postgres)
    }

    fn row_to_json_sqlite(
        &self,
        row: SqliteRow,
    ) -> serde_json::Map<String, Value> {
        row_to_json_impl!(self, row, sqlx::Sqlite)
    }

    pub async fn query(&self, query: &str) -> Result<()> {
        match &self.db {
            DbPool::Postgres(pool) => {
                let mut rows = sqlx::query(query).fetch(pool);
                while let Some(row) = rows.try_next().await? {
                    let json = self.row_to_json_pg(row);
                    println!("{}", to_string(&json)?)
                }
            }
            DbPool::Sqlite(pool) => {
                let mut rows = sqlx::query(query).fetch(pool);
                while let Some(row) = rows.try_next().await? {
                    let json = self.row_to_json_sqlite(row);
                    println!("{}", to_string(&json)?)
                }
            }
        }

        Ok(())
    }
}
