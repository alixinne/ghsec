//! The `default_workflow_permissions` check ensures that the _Default Workflow Permissions_ in a
//! repository use secure values.
//!
//! There are two settings for _Default Workflow Permissions_:
//! - `default_workflow_permissions`: can be `read` or `write`. Defaults to `write`. This controls
//! whether the default `$GITHUB_TOKEN` provided to workflows has write access to the repository
//! (contents, pull requests, etc.) or only read access. Setting this to `read` forces workflow
//! authors to explicitly enable write access for various scopes, which reduces the attack surface
//! of workflows pushed to the repository.
//! - `can_approve_pull_request_reviews`: can be `true` or `false`. Defaults to `true`. This
//! controls whether workflows (through the principal represented by the default `$GITHUB_TOKEN`)
//! is allowed to approve pull requests. Since approving pull requests through a workflow is a way
//! of circumventing branch protections, setting this to `false` prevents attacks that rely on this
//! behavior.
//!
//! This check will emit warnings for repository where any of these settings do not use a secure
//! value.
//!
//! When running with `--fix`, this will set both these settings to secure values:
//! - `default_workflow_permissions`: `read`
//! - `can_approve_pull_request_reviews`: `false`
//!
//! # Sources
//!
//! - [GitHub Docs](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository)
//! - [GitHub REST API](https://docs.github.com/en/rest/actions/permissions?apiVersion=2022-11-28#get-default-workflow-permissions-for-a-repository)

use anyhow::anyhow;
use async_trait::async_trait;
use octocrab::models::Repository;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::{Check, CheckCtx};

#[derive(Debug, Serialize, Deserialize)]
struct DefaultRepositoryWorkflowPermissions {
    // read or write
    default_workflow_permissions: String,
    can_approve_pull_request_reviews: bool,
}

/// Implementation for the `default_workflow_permissions` check
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
