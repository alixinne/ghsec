use anyhow::anyhow;
use async_trait::async_trait;
use octocrab::models::Repository;
use serde::{Deserialize, Serialize};
use tracing::{warn, info};

use super::{Check, CheckCtx};

#[derive(Debug, Serialize, Deserialize)]
struct DefaultRepositoryWorkflowPermissions {
    // read or write
    default_workflow_permissions: String,
    can_approve_pull_request_reviews: bool,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DefaultWorkflowPermissions;

#[async_trait]
impl Check for DefaultWorkflowPermissions {
    #[tracing::instrument(name = "default_workflow_permissions", level = "info", skip_all)]
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()> {
        let route = format!(
            "/repos/{}/actions/permissions/workflow",
            repository
                .full_name
                .as_ref()
                .ok_or_else(|| anyhow!("missing full_name"))?
        );

        let mut fix_needed = false;

        let permissions: DefaultRepositoryWorkflowPermissions =
            ctx.gh.get(&route, Option::<()>::None.as_ref()).await?;

        if permissions.can_approve_pull_request_reviews {
            warn!("can_approve_pull_request_reviews is set to true");
            fix_needed = true;
        }

        if permissions.default_workflow_permissions != "read" {
            warn!(
                "default_workflow_permissions is set to {}",
                permissions.default_workflow_permissions
            );
            fix_needed = true;
        }

        if ctx.args.fix && fix_needed {
            info!("fixing default workflow permissions");

            ctx.gh
                .put(
                    route,
                    Some(&DefaultRepositoryWorkflowPermissions {
                        default_workflow_permissions: "read".to_owned(),
                        can_approve_pull_request_reviews: false,
                    }),
                )
                .await?;
        }

        Ok(())
    }
}
