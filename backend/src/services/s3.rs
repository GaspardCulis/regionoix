use rusty_s3::{Bucket, Credentials};

use crate::utils::get_env_var;

#[derive(Clone)]
pub struct S3Service {
    pub api_bucket: Bucket,
    pub web_bucket: Bucket,
    pub credentials: Credentials,
}

impl S3Service {
    pub fn build() -> anyhow::Result<Self> {
        let secrets = S3Secrets::load()?;

        let api_bucket = Bucket::new(
            secrets
                .endpoint_url
                .parse()
                .expect("endpoint is a valid Url"),
            rusty_s3::UrlStyle::Path,
            secrets.bucket_name.clone(),
            secrets.region.clone(),
        )
        .expect("S3 endpoint URL has a valid scheme and host");

        let web_bucket = Bucket::new(
            secrets
                .web_endpoint_url
                .parse()
                .expect("endpoint is a valid Url"),
            rusty_s3::UrlStyle::VirtualHost,
            secrets.bucket_name.clone(),
            secrets.region.clone(),
        )
        .expect("S3 web endpoint URL has a valid scheme and host");

        let credentials = Credentials::new(
            secrets.access_key.clone(),
            secrets.secret_access_key.clone(),
        );

        Ok(Self {
            api_bucket,
            web_bucket,
            credentials,
        })
    }
}

struct S3Secrets {
    pub endpoint_url: String,
    pub web_endpoint_url: String,
    pub region: String,
    pub bucket_name: String,
    pub access_key: String,
    pub secret_access_key: String,
}

impl S3Secrets {
    fn load() -> anyhow::Result<Self> {
        Ok(Self {
            endpoint_url: get_env_var("S3_ENDPOINT_URL")?,
            web_endpoint_url: get_env_var("S3_WEB_ENDPOINT_URL")?,
            region: get_env_var("S3_REGION")?,
            bucket_name: get_env_var("S3_BUCKET_NAME")?,
            access_key: get_env_var("S3_ACCESS_KEY")?,
            secret_access_key: get_env_var("S3_SECRET_ACCESS_KEY")?,
        })
    }
}
