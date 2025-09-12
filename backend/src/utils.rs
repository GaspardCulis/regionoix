use std::fmt::Debug;
use std::str::FromStr;

use anyhow::anyhow;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PaginateQuery {
    /// Number of results in a page.
    pub page_size: Option<u32>,
    /// Specific page to fetch; page index starts from zero.
    pub page_index: Option<u32>,
}

pub fn get_env_var<T>(env_var_name: &str) -> anyhow::Result<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let var = std::env::var(env_var_name)
        .map_err(|_| anyhow!("Failed to get {} env var", env_var_name))?;
    var.parse::<T>()
        .map_err(|e| anyhow!("Failed to parse {}: {:?}", env_var_name, e))
}
