# ghsec

## ghsec

[![main](https://github.com/vtavernier/ghsec/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/vtavernier/ghsec/actions/workflows/main.yml)

ghsec is an opinionated linter (with fixes) for public GitHub repository security. It helps
diagnose and fix potential security issues caused by GitHub repository settings that are
usually too open by default.

### Installation

#### From source

```bash
cargo install --git https://github.com/vtavernier/ghsec.git
```

#### With [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall --git https://github.com/vtavernier/ghsec.git ghsec
```

### Usage

You will need a personal access token with admin access level to your repositories. Currently,
this tool has only been tested with classic tokens with the repo scope.

```bash
# Provide a GitHub personal access token with admin access to your repositories
export GITHUB_TOKEN=ghp_.....

# Run the checks
ghsec

# Run the checks and fix the issues, if possible
ghsec --fix
```

### Supported checks

- [`default_workflow_permissions`](checks/default_worfklow_permissions/index.html): use secure
defaults for "Default Workflow Permissions"
- [`repository_secrets`](checks/repository_secrets/index.html): list repositories containing
GitHub Actions secrets
