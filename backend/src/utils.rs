use std::fmt::Debug;
use std::str::FromStr;

use anyhow::anyhow;
use sea_orm::{ConnectionTrait, DbErr, PaginatorTrait, Selector, SelectorTrait};
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct PaginateQuery {
    /// Number of results in a page. Defaults to all results.
    pub page_size: Option<u32>,
    /// Specific page to fetch; page index starts from zero. Defaults to zero.
    pub page_index: Option<u32>,
}

impl PaginateQuery {
    pub async fn paginate<'db, C, S>(
        &self,
        selector: Selector<S>,
        db: &'db C,
    ) -> Result<Vec<S::Item>, DbErr>
    where
        C: ConnectionTrait,
        S: SelectorTrait + Send + Sync + 'db,
    {
        match self.page_size {
            Some(page_size) => {
                selector
                    .paginate(db, page_size as u64)
                    .fetch_page(self.page_index.unwrap_or(0) as u64)
                    .await
            }
            None => selector.all(db).await,
        }
    }
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
