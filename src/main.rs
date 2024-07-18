//! [![main](https://github.com/alixinne/ghsec/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/alixinne/ghsec/actions/workflows/main.yml)
//!
//! ghsec is an opinionated linter (with fixes) for public GitHub repository security. It helps
//! diagnose and fix potential security issues caused by GitHub repository settings that are
//! usually too open by default.
//!
//! ## Installation
//!
//! ### From source
//!
//! ```bash
//! cargo install --force --locked ghsec
//! ```
//!
//! ### With [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
//!
//! ```bash
//! cargo binstall ghsec
//! ```
//!
//! ## Usage
//!
//! You will need a personal access token with admin access level to your repositories. Currently,
//! this tool has only been tested with classic tokens with the repo scope.
//!
//! ```bash
//! # Provide a GitHub personal access token with admin access to your repositories
//! export GITHUB_TOKEN=ghp_.....
//!
//! # Run the checks
//! ghsec
//!
//! # Run the checks and fix the issues, if possible
//! ghsec --fix
//!
//! # You can also specify repositories to check using a unix-style glob
//! ghsec 'workflows-*'
//! ```
//!
//! ## Supported checks
//!
//! - [`branch_protections`](https://alixinne.github.io/ghsec/ghsec/checks/branch_protections/index.html):
//! check branch protection settings
//! - [`code_review_limits`](https://alixinne.github.io/ghsec/ghsec/checks/code_review_limits/index.html):
//! check account settings for code review limits
//! - [`default_workflow_permissions`](https://alixinne.github.io/ghsec/ghsec/checks/default_worfklow_permissions/index.html):
//! use secure defaults for "Default Workflow Permissions"
//! - [`fork_pull_request_workflows`](https://alixinne.github.io/ghsec/ghsec/checks/fork_pull_request_workflows/index.html):
//! check repository settings for public fork pull request workflow runs
//! - [`repository_secrets`](https://alixinne.github.io/ghsec/ghsec/checks/repository_secrets/index.html):
//! list repositories containing GitHub Actions secrets

use std::str::FromStr;

use clap::Parser;
use futures_util::{stream::FuturesUnordered, StreamExt, TryStreamExt};
use octocrab::{models::Repository, Octocrab};
use tokio::pin;
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{filter::Directive, EnvFilter};

mod args;
use args::Args;

pub mod checks;
use checks::{AccountCheck, CheckCtx, Checks, RepositoryCheck};

#[tracing::instrument(name="repository", level="info", skip_all, fields(repository = repository.full_name.as_ref().unwrap()))]
async fn process_repo<'c>(ctx: &'c CheckCtx<'c>, repository: Repository) -> anyhow::Result<()> {
    for check in ctx.args.checks.clone().into_iter() {
        if let Checks::Repository(check) = check {
            debug!(check = %check, "running check");
            check.run(ctx, &repository).await?;
        }
    }

    Ok(())
}

#[tracing::instrument(name = "account", level = "info", skip_all)]
async fn process_account<'c>(ctx: &'c CheckCtx<'c>) -> anyhow::Result<()> {
    for check in ctx.args.checks.clone().into_iter() {
        if let Checks::Account(check) = check {
            debug!(check = %check, "running check");
            check.run(ctx).await?;
        }
    }

    Ok(())
}

#[tokio::main(worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // Load variables from .env
    dotenv::dotenv().ok();

    // Load arguments
    let args = Args::parse();

    let filter = EnvFilter::builder()
        .with_default_directive(if args.debug {
            Directive::from_str("ghsec=debug").unwrap()
        } else {
            LevelFilter::INFO.into()
        })
        .from_env_lossy();

    let fmt = tracing_subscriber::fmt().with_env_filter(filter);
    if args.json {
        fmt.json().init()
    } else {
        fmt.compact().init();
    };

    // Create client
    let gh = Octocrab::builder()
        .personal_token(args.github_token.unsecure().to_string())
        .build()?;

    // Print authentication information
    let current_user = gh.current();
    info!("Logged in as {}", current_user.user().await?.login);

    // Context for running checks
    let ctx = CheckCtx::new(&args, &gh);

    if args.checks.has_repository_checks() {
        // Get target repositories
        let repos = current_user
            .list_repos_for_authenticated_user()
            .type_("owner")
            .send()
            .await?
            .into_stream(&gh);
        pin!(repos);

        // Build a FuturesUnordered
        let mut tasks = FuturesUnordered::new();
        while let Some(target_repo) = repos.try_next().await? {
            if args.repository_names.matches(&target_repo.name) {
                tasks.push(process_repo(&ctx, target_repo));
            } else {
                debug!(repository = %target_repo.name, "skipping repository not matching input pattern");
            }
        }

        // Poll it
        while let Some(result) = tasks.next().await {
            if let Err(err) = result {
                error!(?err, "error running check");
            }
        }
    }

    // Finish with account-level tasks
    process_account(&ctx).await
}
