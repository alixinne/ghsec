use std::str::FromStr;

use clap::Parser;
use secure_string::SecureString;
use strum::IntoEnumIterator;

use crate::checks::Checks;

#[derive(Debug, Clone)]
pub enum CheckRunRequest {
    All,
    Specific(Vec<Checks>),
}

impl Default for CheckRunRequest {
    fn default() -> Self {
        Self::All
    }
}

impl FromStr for CheckRunRequest {
    type Err = <Checks as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut results = vec![];

        for part in s.split(',') {
            let without_whitespace = part.trim().to_lowercase();

            // If there's one "All" spec, return this
            if without_whitespace == "all" {
                return Ok(Self::All);
            }

            results.push(Checks::from_str(&without_whitespace)?);
        }

        Ok(Self::Specific(results))
    }
}

pub enum CheckRunRequestIterator {
    All(<Checks as strum::IntoEnumIterator>::Iterator),
    Specific(<Vec<Checks> as IntoIterator>::IntoIter),
}

impl Iterator for CheckRunRequestIterator {
    type Item = Checks;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CheckRunRequestIterator::All(ref mut inner) => inner.next(),
            CheckRunRequestIterator::Specific(ref mut inner) => inner.next(),
        }
    }
}

impl IntoIterator for CheckRunRequest {
    type Item = Checks;

    type IntoIter = CheckRunRequestIterator;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            CheckRunRequest::All => <Self as IntoIterator>::IntoIter::All(Checks::iter()),
            CheckRunRequest::Specific(selected) => {
                <Self as IntoIterator>::IntoIter::Specific(selected.into_iter())
            }
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
}
