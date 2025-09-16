use stripe::Client;

use crate::utils::get_env_var;

#[derive(Clone)]
pub struct StripeService {
    pub client: Client,
    pub api_key: String,
}

impl StripeService {
    pub fn build() -> anyhow::Result<Self> {
        let secrets = StripeSecrets::load()?;

        let client = Client::new(secrets.api_key.clone());

        Ok(Self {
            client,
            api_key: secrets.api_key,
        })
    }
}

struct StripeSecrets {
    pub api_key: String,
}

impl StripeSecrets {
    fn load() -> anyhow::Result<Self> {
        Ok(Self {
            api_key: get_env_var("STRIPE_API_KEY")?,
        })
    }
}
