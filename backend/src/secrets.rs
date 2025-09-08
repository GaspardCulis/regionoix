use dotenv::dotenv;

pub struct Secrets {
    pub api_host: String,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub secret_key: String,
}

impl Secrets {
    pub fn load() -> anyhow::Result<Self> {
        if dotenv().is_err() {
            println!("Failed to read .env, falling back to existing env vars");
        }

        Ok(Self {
            api_host: std::env::var("API_HOST")?,
            api_port: std::env::var("API_PORT")?.parse()?,
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,
            secret_key: std::env::var("SECRET_KEY")?,
        })
    }
}
