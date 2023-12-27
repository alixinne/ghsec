use std::str::FromStr;

use anyhow::bail;
use clap::Parser;
use secure_string::SecureString;
use strum::IntoEnumIterator;

use crate::checks::{AccountChecks, Checks, RepositoryChecks};

#[derive(Debug, Clone)]
pub enum CheckRunRequest {
    All,
    Specific(Vec<Checks>),
}

impl CheckRunRequest {
    pub fn has_repository_checks(&self) -> bool {
        match self {
            CheckRunRequest::All => true,
            CheckRunRequest::Specific(checks) => checks
                .iter()
                .any(|check| matches!(check, Checks::Repository(_))),
        }
    }
}

impl Default for CheckRunRequest {
    fn default() -> Self {
        Self::All
    }
}

impl FromStr for CheckRunRequest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut results = vec![];

        for part in s.split(',') {
            let without_whitespace = part.trim().to_lowercase();

            // If there's one "All" spec, return this
            if without_whitespace == "all" {
                return Ok(Self::All);
            }

            if let Ok(check) = AccountChecks::from_str(&without_whitespace) {
                results.push(check.into());
            } else if let Ok(check) = RepositoryChecks::from_str(&without_whitespace) {
                results.push(check.into());
            } else {
                bail!("unknown check type")
            }
        }

        Ok(Self::Specific(results))
    }
}

impl IntoIterator for CheckRunRequest {
    type Item = Checks;

    type IntoIter = <Vec<Checks> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CheckRunRequest::All => {
                let mut result = vec![];
                result.extend(RepositoryChecks::iter().map(Into::into));
                result.extend(AccountChecks::iter().map(Into::into));
                result.into_iter()
            }
            CheckRunRequest::Specific(selected) => selected.into_iter(),
        }
    }
}

#[derive(Debug, Parser)]
pub struct Args {
    /// GitHub Personal Access Token
    #[arg(long, env = "GITHUB_TOKEN")]
    pub github_token: SecureString,

    /// Should we fix things?
    #[arg(long)]
    pub fix: bool,

    /// Which checks to run
    #[arg(short = 'C', long, default_value = "all")]
    pub checks: CheckRunRequest,

    /// Output logs as JSON
    #[arg(long)]
    pub json: bool,

    /// Enable debug logs
    #[arg(short = 'D', long)]
    pub debug: bool,

    /// Target repositories to run checks on. Supports globs.
    #[arg(default_value = "*")]
    pub repository_names: glob::Pattern,

    /// Which secrets should trigger warnings instead of regular info
    #[arg(long, default_value = "^GH_(TOKEN|PAT)$")]
    pub repository_secrets_warn_secret_names: regex::Regex,
}
