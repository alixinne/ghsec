//! The `code_review_limits` check ensures that the _Code review limits_ in a
//! repository use secure values.
//!
//! By default, public GitHub repositories allow anyone to submit reviews that approve or request
//! changes to a pull requests. This means that anyone without any particularly privileged access
//! could enable a pull request to be merged, as long as it was created by another user or tool,
//! and set up for auto-merge in one way or another.
//!
//! This check will print an error with a link which can be followed to enable _Code review limits_
//! on the target repository, or globally for the account. Since apparently this "internal hackaton
//! project" (see the
//! [announcement](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-pull-request-reviews-in-your-repository))
//! got pushed to production without thinking about adding an API, there is no way to automate this
//! without horrible hacks for now.
//!
//! # Sources
//!
//! - [GitHub Docs](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-pull-request-reviews-in-your-repository)
//! - [Blog Announcement](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-pull-request-reviews-in-your-repository)

use async_trait::async_trait;
use tracing::error;

use super::{AccountCheck, CheckCtx};

/// Implementation for the `default_workflow_permissions` check
#[derive(Default, Debug, Clone, Copy)]
pub struct CodeReviewLimits;

#[async_trait]
impl AccountCheck for CodeReviewLimits {
    #[tracing::instrument(name = "default_workflow_permissions", level = "info", skip_all)]
    async fn run<'c>(&self, _ctx: &'c CheckCtx<'c>) -> anyhow::Result<()> {
        error!("ghsec cannot programatically check or change settings for Code Review Limits. Go to https://github.com/settings/code_review_limits and make sure that the option is enabled.");
        Ok(())
    }
}
