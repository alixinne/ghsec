//! The `branch_protections` checks the settings on the configured branch protections of a
//! repository.
//!
//! Branch protections are available for free on all public repositories, and on private
//! repositories with paid plans. They protect matching branches against accidental merges, pushes,
//! deletion and other potentially destructive operations. They are also used for supporting the
//! _auto-merge_ feature for pull requests. Not having branch protections configured, or having
//! them configured _incorrectly_ allows repository owners to make mistakes, or let automated tools
//! break the default branch by pushing incompatible changes.
//!
//! Due to limitations in the REST API, only branch protections with matching branches can be
//! discovered and checked. For simplicity, this check currently only checks the default branch.
//!
//! When running with `--fix`, this check currently does not do anything.
//!
//! # Sources
//!
//! - [GitHub Docs](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
//! - [GitHub REST API](https://docs.github.com/en/rest/branches/branch-protection?apiVersion=2022-11-28)

use anyhow::anyhow;
use async_trait::async_trait;
use octocrab::models::Repository;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use super::{CheckCtx, RepositoryCheck};

#[derive(Debug, Serialize, Deserialize)]
struct BranchProtection {
    required_status_checks: Option<RequiredStatusChecks>,
    enforce_admins: ProtectionFlag,
    required_pull_request_reviews: Option<RequiredPullRequestReviews>,
    restrictions: Option<Restrictions>,
    required_linear_history: ProtectionFlag,
    allow_force_pushes: ProtectionFlag,
    allow_deletions: ProtectionFlag,
    required_conversation_resolution: ProtectionFlag,
    lock_branch: ProtectionFlag,
    allow_fork_syncing: ProtectionFlag,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequiredStatusChecks {
    contexts: Vec<String>,
    enforcement_level: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProtectionFlag {
    enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequiredPullRequestReviews {
    dismissal_restrictions: Option<Restrictions>,
    dismiss_stale_reviews: bool,
    require_code_owner_reviews: bool,
    required_approving_review_count: i32,
    require_last_push_approval: bool,
}

// TODO: Implement sub-models
#[derive(Debug, Serialize, Deserialize)]
struct Restrictions {
    users: Vec<serde_json::Value>,
    teams: Vec<serde_json::Value>,
    apps: Vec<serde_json::Value>,
}

/// Implementation for the `repository_secrets` check
#[derive(Default, Debug, Clone, Copy)]
pub struct BranchProtections;

#[async_trait]
impl RepositoryCheck for BranchProtections {
    #[tracing::instrument(name = "branch_protections", level = "info", skip_all)]
    async fn run<'c>(&self, ctx: &'c CheckCtx<'c>, repository: &Repository) -> anyhow::Result<()> {
        if let Some(default_branch) = &repository.default_branch {
            let full_name = repository
                .full_name
                .as_ref()
                .ok_or_else(|| anyhow!("missing repository full name"))?;

            let protection = ctx
                .gh
                .get::<BranchProtection, _, _>(
                    format!("/repos/{full_name}/branches/{default_branch}/protection"),
                    Option::<()>::None.as_ref(),
                )
                .await;

            match protection {
                Ok(protection) => {
                    if let Some(required_status_checks) = &protection.required_status_checks {
                        if required_status_checks.contexts.is_empty() {
                            warn!("no contexts configured for required status checks");
                        }
                    } else {
                        warn!("no required checks configured for branch protection");
                    }

                    if !protection.enforce_admins.enabled {
                        warn!("branch protection not enforced for admins");
                    }

                    if protection.required_pull_request_reviews.is_none() {
                        warn!("branch protection does not require pull requests");
                    }

                    if protection.allow_force_pushes.enabled {
                        warn!("branch protection allows force pushes");
                    }

                    if protection.allow_deletions.enabled {
                        warn!("branch protection allows deletions");
                    }

                    if ctx.args.fix {
                        info!("nothing to fix regarding branch protections yet");
                    }
                }
                Err(err) => {
                    let mut handled = false;

                    if let octocrab::Error::GitHub { source, .. } = &err {
                        if source.message == "Branch not protected" {
                            // TODO: Allow auto-fixing this
                            let link = format!("https://github.com/{full_name}/settings/branches");
                            warn!(link, "missing branch protection on default branch, you should configure one at {link}");
                            handled = true;

                            if ctx.args.fix {
                                info!("nothing to fix regarding branch protections yet");
                            }
                        } else if source.message.contains("Upgrade to GitHub Pro") {
                            warn!("feature not available for this repository type");
                            handled = true;
                        }
                    }

                    if !handled {
                        return Err(err.into());
                    }
                }
            }
        } else {
            warn!("missing default branch information, cannot check branch protections for it");
        }

        Ok(())
    }
}
