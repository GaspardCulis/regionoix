use std::ops::Deref;

use meilisearch_sdk::client::Client;

use crate::utils::get_env_var;

#[derive(Clone)]
pub struct SearchService(Client);

impl SearchService {
    pub fn build_search() -> anyhow::Result<Self> {
        let secrets = MeiliSecrets::load()?;

        let search = Client::new(secrets.api_url, Some(secrets.search_api_key))?;

        Ok(Self(search))
    }

    pub fn build_admin() -> anyhow::Result<Self> {
        let secrets = MeiliSecrets::load()?;

        let search = Client::new(secrets.api_url, Some(secrets.admin_api_key))?;

        Ok(Self(search))
    }
}

impl Deref for SearchService {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MeiliSecrets {
    pub api_url: String,
    pub admin_api_key: String,
    pub search_api_key: String,
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
