use clap::Parser;
use futures_util::{stream::FuturesUnordered, StreamExt, TryStreamExt};
use octocrab::{models::Repository, Octocrab};
use secure_string::SecureString;
use tokio::pin;
use tracing::{debug, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

mod checks;
use checks::{Check, CheckCtx, DefaultWorkflowPermissions, RepositorySecrets};

#[derive(Debug, Parser)]
pub struct Args {
    /// GitHub Personal Access Token
    #[arg(long, env = "GITHUB_TOKEN")]
    github_token: SecureString,

    /// Should we fix things?
    #[arg(long)]
    fix: bool,
}

#[tracing::instrument(name="repository", level="info", skip_all, fields(repository = repository.full_name.as_ref().unwrap()))]
async fn process_repo<'c>(ctx: &'c CheckCtx<'c>, repository: Repository) -> anyhow::Result<()> {
    debug!("inspecting default workflow permission");

    DefaultWorkflowPermissions.run(ctx, &repository).await?;

    debug!("inspecting secrets");

    RepositorySecrets.run(ctx, &repository).await?;

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

    // Context for running checks
    let ctx = CheckCtx::new(&args, &gh);

    // Build a FuturesUnordered
    let mut tasks = FuturesUnordered::new();
    while let Some(target_repo) = repos.try_next().await? {
        tasks.push(process_repo(&ctx, target_repo));
    }

    // Poll it
    while tasks.next().await.is_some() {}

    Ok(())
}
