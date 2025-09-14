use std::ops::Deref;

use sea_orm::{Database, DatabaseConnection};

use crate::utils::get_env_var;

#[derive(Clone)]
pub struct DatabaseService {
    pub conn: DatabaseConnection,
}

impl DatabaseService {
    pub async fn build() -> anyhow::Result<Self> {
        let secrets = DatabaseSecrets::load()?;

        let db = Database::connect(&secrets.database_url).await?;

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
