use std::ops::Deref;

use sea_orm::*;
use tracing::info;

use crate::utils::get_env_var;

#[derive(Clone)]
pub struct DatabaseService {
    pub conn: DatabaseConnection,
}

impl DatabaseService {
    pub async fn build() -> anyhow::Result<Self> {
        info!("Connecting to DatabaseService");
        let secrets = DatabaseSecrets::load()?;

        let db = Database::connect(&secrets.database_url).await?;

        Ok(Self { conn: db })
    }

    pub async fn build_integration_test<F>(schema_builder: F) -> anyhow::Result<Self>
    where
        F: FnOnce(Schema, &DbBackend) -> Vec<Statement>,
    {
        let db = Database::connect("sqlite::memory:").await?;

        let schema = Schema::new(DbBackend::Sqlite);
        let statements = schema_builder(schema, &DbBackend::Sqlite);
        for stmt in statements {
            db.execute(stmt).await?;
        }

        Ok(Self { conn: db })
    }
}

impl Deref for DatabaseService {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

#[derive(Clone)]
struct DatabaseSecrets {
    pub database_url: String,
}

impl DatabaseSecrets {
    fn load() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: get_env_var("DATABASE_URL")?,
        })
    }
}
