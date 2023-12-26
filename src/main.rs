use anyhow::anyhow;
use chrono::{DateTime, Utc};
use clap::Parser;
use futures_util::{stream::FuturesUnordered, StreamExt, TryStreamExt};
use octocrab::{models::Repository, Octocrab};
use secure_string::SecureString;
use serde::{Deserialize, Serialize};
use tokio::pin;
use tracing::{debug, info, level_filters::LevelFilter, warn};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
struct Args {
    /// GitHub Personal Access Token
    #[arg(long, env = "GITHUB_TOKEN")]
    github_token: SecureString,

    /// Should we fix things?
    #[arg(long)]
    fix: bool,
}

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

#[derive(Debug, Serialize, Deserialize)]
struct DefaultRepositoryWorkflowPermissions {
    // read or write
    default_workflow_permissions: String,
    can_approve_pull_request_reviews: bool,
}

#[tracing::instrument(name="repository", level="info", skip_all, fields(repository = target_repo.full_name.as_ref().unwrap()))]
async fn process_repo(gh: Octocrab, target_repo: Repository, args: &Args) -> anyhow::Result<()> {
    debug!("inspecting default workflow permission");

    let route = format!(
        "/repos/{}/actions/permissions/workflow",
        target_repo
            .full_name
            .ok_or_else(|| anyhow!("missing full_name"))?
    );

    let permissions: DefaultRepositoryWorkflowPermissions =
        gh.get(&route, Option::<()>::None.as_ref()).await?;

    if permissions.can_approve_pull_request_reviews {
        warn!("can_approve_pull_request_reviews is set to true");
    }

    if permissions.default_workflow_permissions != "read" {
        warn!(
            "default_workflow_permissions is set to {}",
            permissions.default_workflow_permissions
        );
    }

    if args.fix {
        gh.put(
            route,
            Some(&DefaultRepositoryWorkflowPermissions {
                default_workflow_permissions: "read".to_owned(),
                can_approve_pull_request_reviews: false,
            }),
        )
        .await?;
    }

    debug!("inspecting secrets");

    let secrets: SecretList = gh
        .get(
            format!(
                "/repos/{}/{}/actions/secrets",
                target_repo
                    .owner
                    .ok_or_else(|| anyhow!("missing owner"))?
                    .login,
                target_repo.name
            ),
            Option::<()>::None.as_ref(),
        )
        .await?;

    for secret in secrets.secrets {
        info!(secret_name = secret.name, "found secret");
    }

    Ok(())
}

#[tokio::main(worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // Load variables from .env
    dotenv::dotenv().ok();

    // Load arguments
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    // Create client
    let gh = Octocrab::builder()
        .personal_token(args.github_token.unsecure().to_string())
        .build()?;

    // Print authentication information
    let current_user = gh.current();
    info!("Logged in as {}", current_user.user().await?.login);

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
        tasks.push(process_repo(gh.clone(), target_repo, &args));
    }

    // Poll it
    while tasks.next().await.is_some() {}

    Ok(())
}
