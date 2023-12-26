use anyhow::anyhow;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use octocrab::models::Repository;
use serde::{Deserialize, Serialize};
use tracing::info;

use super::{Check, CheckCtx};

#[derive(Debug, Serialize, Deserialize)]
struct SecretList {
    total_count: i32,
    secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Secret {
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct RepositorySecrets;

#[async_trait]
impl Check for RepositorySecrets {
    #[tracing::instrument(name = "repository_secrets", level = "info", skip_all)]
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()> {
        let secrets: SecretList = ctx
            .gh
            .get(
                format!(
                    "/repos/{}/{}/actions/secrets",
                    repository
                        .owner
                        .as_ref()
                        .ok_or_else(|| anyhow!("missing owner"))?
                        .login,
                    repository.name
                ),
                Option::<()>::None.as_ref(),
            )
            .await?;

        for secret in secrets.secrets {
            info!(secret_name = secret.name, "found secret");
        }

        Ok(())
    }
}
