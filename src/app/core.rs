use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use futures::future::try_join_all;
use tokio::{fs::File, io::AsyncReadExt, spawn};

use crate::{ConnectionString, Db};

pub struct App {
    pub databases: Vec<Arc<Db>>,
    pub path_to_query: PathBuf,
}

impl App {
    pub async fn new(
        connection_strings: Vec<ConnectionString>,
        path_to_query: PathBuf,
    ) -> Result<Self> {
        let mut databases = Vec::with_capacity(connection_strings.len());
        let futures =
            connection_strings.into_iter().map(|connection_string| {
                spawn(async move {
                    let database: Db = Db::new(connection_string).await?;

                    Ok::<_, anyhow::Error>(Arc::new(database))
                })
            });

        let database_results = try_join_all(futures).await?;
        for database in database_results {
            databases.push(database?)
        }

        Ok(Self { databases, path_to_query })
    }

    pub async fn execute_query_from_file(&self) -> Result<()> {
        let query = Arc::new(self.load_query_from_file().await?);

        let futures = self.databases.iter().map(|db| {
            let query = query.clone();
            let db = db.clone();

            spawn(async move { db.query(query.as_str()).await })
        });

        let results = try_join_all(futures).await?;

        for result in results {
            result?;
        }

        Ok(())
    }

    pub async fn load_query_from_file(&self) -> Result<String> {
        let mut file = File::open(&self.path_to_query).await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;

        Ok(contents)
    }
}
