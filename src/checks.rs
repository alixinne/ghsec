use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use octocrab::{Octocrab, models::Repository};

mod default_worfklow_permissions;
pub use default_worfklow_permissions::*;

mod repository_secrets;
pub use repository_secrets::*;

use crate::Args;

// Context for running a check against GitHub
pub struct CheckCtx<'c> {
    pub args: &'c Args,
    pub gh: &'c Octocrab,
}

impl<'c> CheckCtx<'c> {
    pub fn new(args: &'c Args, gh: &'c Octocrab) -> Self {
        Self { args, gh }
    }
}

#[async_trait]
#[enum_dispatch]
pub trait Check {
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()>;
}

#[enum_dispatch(Check)]
pub enum Checks {
    DefaultWorkflowPermissions,
    RepositorySecrets,
}
