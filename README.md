# [ghsec](https://github.com/alixinne/ghsec)

[![main](https://github.com/alixinne/ghsec/actions/workflows/main.yml/badge.svg?event=push)](https://github.com/alixinne/ghsec/actions/workflows/main.yml)

ghsec is an opinionated linter (with fixes) for public GitHub repository security. It helps
diagnose and fix potential security issues caused by GitHub repository settings that are
usually too open by default.

### Installation

#### From source

```bash
cargo install --force --locked ghsec
```

#### With [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall ghsec
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

# You can also specify repositories to check using a unix-style glob
ghsec 'workflows-*'
```

### Supported checks

- [`branch_protections`](https://alixinne.github.io/ghsec/ghsec/checks/branch_protections/index.html):
check branch protection settings
- [`code_review_limits`](https://alixinne.github.io/ghsec/ghsec/checks/code_review_limits/index.html):
check account settings for code review limits
- [`default_workflow_permissions`](https://alixinne.github.io/ghsec/ghsec/checks/default_worfklow_permissions/index.html):
use secure defaults for "Default Workflow Permissions"
- [`fork_pull_request_workflows`](https://alixinne.github.io/ghsec/ghsec/checks/fork_pull_request_workflows/index.html):
check repository settings for public fork pull request workflow runs
- [`repository_secrets`](https://alixinne.github.io/ghsec/ghsec/checks/repository_secrets/index.html):
list repositories containing GitHub Actions secrets

## License

This project is licensed under the [MIT License](LICENSE).
