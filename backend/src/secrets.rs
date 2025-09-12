use std::fmt::Debug;
use std::str::FromStr;

use anyhow::anyhow;
use dotenv::dotenv;

pub struct Secrets {
    pub api_host: String,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub secret_key: String,
    pub meili: MeiliSecrets,
    pub s3: S3Secrets,
}

pub struct MeiliSecrets {
    pub api_url: String,
    pub admin_api_key: String,
    pub search_api_key: String,
}

pub struct S3Secrets {
    pub endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_access_key: String,
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        if dotenv().is_err() {
            println!("Failed to read .env, falling back to existing env vars");
        }

        Ok(Self {
            api_host: get_env_var("API_HOST")?,
            api_port: get_env_var("API_PORT")?,
            database_url: get_env_var("DATABASE_URL")?,
            redis_url: get_env_var("REDIS_URL")?,
            secret_key: get_env_var("SECRET_KEY")?,
            meili: MeiliSecrets::load()?,
            s3: S3Secrets::load()?,
        })
    }
}

impl MeiliSecrets {
    fn load() -> anyhow::Result<Self> {
        Ok(Self {
            api_url: get_env_var("MEILISEARCH_URL")?,
            admin_api_key: get_env_var("MEILISEARCH_ADMIN_KEY")?,
            search_api_key: get_env_var("MEILISEARCH_SEARCH_KEY")?,
        })
    }
}

impl S3Secrets {
    fn load() -> anyhow::Result<Self> {
        Ok(Self {
            endpoint_url: get_env_var("S3_ENDPOINT_URL")?,
            region: get_env_var("S3_REGION")?,
            bucket_name: get_env_var("S3_BUCKET_NAME")?,
            access_key: get_env_var("S3_ACCESS_KEY")?,
            secret_access_key: get_env_var("S3_SECRET_ACCESS_KEY")?,
        })
    }
}

fn get_env_var<T>(env_var_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let var = std::env::var(env_var_name)
        .map_err(|_| anyhow!("Failed to get {} env var", env_var_name))?;
    var.parse::<T>()
        .map_err(|e| anyhow!("Failed to parse {}: {:?}", env_var_name, e))
}
