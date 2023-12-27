//! The `fork_pull_request_workflows` check ensures that the _Fork pull request workflows from
//! outside collaborators_ setting uses a secure value.
//!
//! Allowing outside contributors to run CI workflows without being a known contributor or without
//! approval is a security risk, since workflows that run in pull requests can be changed by the
//! pull request triggering the workflow itself.
//!
//! "Require approval for first-time contributors" should be the default in recently created
//! repositories, and is a secure option since, as the name implies, requires approval to run
//! workflows in pull requests made by new contributors.
//!
//! Again, GitHub added pushed this feature to production without a corresponding API (see the
//! [discussion](https://github.com/orgs/community/discussions/35808)), so all this check can do is
//! print a reminder with a link to the settings page for the corresponding repository.
//!
//! # Sources
//!
//! - [GitHub Docs](https://docs.github.com/en/actions/managing-workflow-runs/approving-workflow-runs-from-public-forks)
//! - [Missing API Endpoint discussion](https://github.com/orgs/community/discussions/35808)

use anyhow::anyhow;
use async_trait::async_trait;
use octocrab::models::Repository;
use tracing::error;

use super::{CheckCtx, RepositoryCheck};

/// Implementation for the `fork_pull_request_workflows` check
#[derive(Default, Debug, Clone, Copy)]
pub struct ForkPullRequestWorkflows;

#[async_trait]
impl RepositoryCheck for ForkPullRequestWorkflows {
    #[tracing::instrument(name = "fork_pull_request_workflows", level = "info", skip_all)]
    async fn run<'c>(&self, _ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()> {
        let link = format!(
            "https://github.com/{}/settings/actions",
            repository
                .full_name
                .as_ref()
                .ok_or_else(|| anyhow!("missing repository full name"))?
        );
        error!(link, "ghsec cannot programatically check or change settings for 'Fork pull request workflows from outside collaborators'. Go to {link} and make sure that the 'Require approval for first-time contributors' option is selected.");
        Ok(())
    }
}
