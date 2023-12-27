//! Implementation for security checks on repositories

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use octocrab::{models::Repository, Octocrab};

mod code_review_limits;
pub use code_review_limits::*;

mod default_worfklow_permissions;
pub use default_worfklow_permissions::*;

mod fork_pull_request_workflows;
pub use fork_pull_request_workflows::*;

mod repository_secrets;
pub use repository_secrets::*;

use crate::Args;

/// Context for running a check against GitHub
pub struct CheckCtx<'c> {
    /// Arguments to the CLI
    pub args: &'c Args,
    /// GitHub API client
    pub gh: &'c Octocrab,
}

impl<'c> CheckCtx<'c> {
    pub fn new(args: &'c Args, gh: &'c Octocrab) -> Self {
        Self { args, gh }
    }
}

/// Represents the possible operations for a repository check
#[async_trait]
#[enum_dispatch]
pub trait RepositoryCheck {
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()>;
}

/// Represents the possible operations for an account check
#[async_trait]
#[enum_dispatch]
pub trait AccountCheck {
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>) -> anyhow::Result<()>;
}

/// Represents all the available checks on a repository
#[enum_dispatch(RepositoryCheck)]
#[derive(Debug, Clone, strum::EnumIter, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum RepositoryChecks {
    DefaultWorkflowPermissions,
    ForkPullRequestWorkflows,
    RepositorySecrets,
}

/// Represents all the available checks on an account
#[enum_dispatch(AccountCheck)]
#[derive(Debug, Clone, strum::EnumIter, strum::EnumString, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum AccountChecks {
    CodeReviewLimits,
}

/// Represents all the available checks
#[derive(Debug, Clone)]
pub enum Checks {
    Repository(RepositoryChecks),
    Account(AccountChecks),
}

impl From<RepositoryChecks> for Checks {
    fn from(value: RepositoryChecks) -> Self {
        Self::Repository(value)
    }
}

impl From<AccountChecks> for Checks {
    fn from(value: AccountChecks) -> Self {
        Self::Account(value)
    }
}
