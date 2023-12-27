# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.5.0](https://github.com/vtavernier/ghsec/compare/v0.4.0..v0.5.0) - 2023-12-27
#### Bug Fixes
- expose clickable links as tracing fields - ([18678f1](https://github.com/vtavernier/ghsec/commit/18678f191cd3d8118123b192677704323a958b5b)) - [@vtavernier](https://github.com/vtavernier)
#### Continuous Integration
- publish to crates.io - ([56cfc7b](https://github.com/vtavernier/ghsec/commit/56cfc7bd37f4f66f1d883c7e45961cbc5063240b)) - [@vtavernier](https://github.com/vtavernier)
#### Features
- implement basic branch_protections check - ([ed8ac90](https://github.com/vtavernier/ghsec/commit/ed8ac908ad89b6550539353ec270e83fa54b38a5)) - [@vtavernier](https://github.com/vtavernier)

- - -

## [v0.4.0](https://github.com/vtavernier/ghsec/compare/v0.3.0..v0.4.0) - 2023-12-27
#### Continuous Integration
- use cog changelog in releases instead of --generate-notes - ([bb4225a](https://github.com/vtavernier/ghsec/commit/bb4225a6c655673d9e8fdb0baabe52f002605df1)) - [@vtavernier](https://github.com/vtavernier)
#### Documentation
- **(code_review_limits)** fix typo - ([f1d5e8a](https://github.com/vtavernier/ghsec/commit/f1d5e8af40c61021f8dc1721646f59682d767aef)) - [@vtavernier](https://github.com/vtavernier)
- fix list in readme - ([9addb63](https://github.com/vtavernier/ghsec/commit/9addb634cabc2cfd92d246bd29cf8f89aa33690d)) - [@vtavernier](https://github.com/vtavernier)
#### Features
- add fork_pull_request_workflow check - ([cf98b35](https://github.com/vtavernier/ghsec/commit/cf98b35491bbb90572a63f9235d7c2ae456cfcae)) - [@vtavernier](https://github.com/vtavernier)

- - -

## [v0.3.0](https://github.com/vtavernier/ghsec/compare/v0.2.0..v0.3.0) - 2023-12-27
#### Documentation
- **(checks)** add documentation for current checks - ([bcc10f7](https://github.com/vtavernier/ghsec/commit/bcc10f7fa5b6aaaad8cc0a4dfe70b437a7f03b33)) - [@vtavernier](https://github.com/vtavernier)
- fix links to supported checks - ([1cb20d1](https://github.com/vtavernier/ghsec/commit/1cb20d1742204e32b3dfda8823188671329e014e)) - [@vtavernier](https://github.com/vtavernier)
- add LICENSE - ([4eab024](https://github.com/vtavernier/ghsec/commit/4eab024c9fa9f7607ab342f3af3dcc4c5899b3a9)) - [@vtavernier](https://github.com/vtavernier)
#### Features
- **(repository_secrets)** add secret name regex for triggering warnings - ([f199330](https://github.com/vtavernier/ghsec/commit/f1993301d26a606238d6482ebb4c6d8df69869bd)) - [@vtavernier](https://github.com/vtavernier)
- add code_review_limits check - ([d20ffc3](https://github.com/vtavernier/ghsec/commit/d20ffc3d0865e9336549d881d38b513428a33743)) - [@vtavernier](https://github.com/vtavernier)
- filter target repositories using glob - ([33192c7](https://github.com/vtavernier/ghsec/commit/33192c75f195c70ab76419c57468d29046410dbc)) - [@vtavernier](https://github.com/vtavernier)

- - -

## [v0.2.0](https://github.com/vtavernier/ghsec/compare/v0.1.0..v0.2.0) - 2023-12-26
#### Bug Fixes
- update logging and fix behavior - ([7ca9e04](https://github.com/vtavernier/ghsec/commit/7ca9e041505ff1a78917ecd488031647dad5b24c)) - [@vtavernier](https://github.com/vtavernier)
#### Build system
- set version in Cargo.toml on release - ([e581820](https://github.com/vtavernier/ghsec/commit/e581820b78a6201ac7064f0555fa5d7b18c8a734)) - [@vtavernier](https://github.com/vtavernier)
- generate readme on release - ([aafd22c](https://github.com/vtavernier/ghsec/commit/aafd22cb4830ca9c86b9892280aa4a0d90b75e4d)) - [@vtavernier](https://github.com/vtavernier)
#### Continuous Integration
- enable github pages build - ([f201c1b](https://github.com/vtavernier/ghsec/commit/f201c1b02df6378e8311a965aec2ce2b29a68bef)) - [@vtavernier](https://github.com/vtavernier)
- add GITHUB_TOKEN to environment - ([0b736c4](https://github.com/vtavernier/ghsec/commit/0b736c40eedaaca8b5b100c2dacd8709113db645)) - [@vtavernier](https://github.com/vtavernier)
#### Documentation
- add main module documentation - ([c89ae3c](https://github.com/vtavernier/ghsec/commit/c89ae3cb91bf8638b1868c0e0a83c4cc3193af0f)) - [@vtavernier](https://github.com/vtavernier)
#### Features
- support cargo-binstall - ([14a6169](https://github.com/vtavernier/ghsec/commit/14a6169622afffad0ee7d54a3c0ed6be3e48e104)) - [@vtavernier](https://github.com/vtavernier)
- add json and debug logging options - ([ac90704](https://github.com/vtavernier/ghsec/commit/ac907040c75b16719b92e27326ac44ebe6a7b31a)) - [@vtavernier](https://github.com/vtavernier)
- allow configuring checks to run - ([045b193](https://github.com/vtavernier/ghsec/commit/045b193f4bb1f056f0f5b3e6231461b4452e51c2)) - [@vtavernier](https://github.com/vtavernier)
#### Miscellaneous Chores
- **(gitignore)** ignore built binaries - ([0798581](https://github.com/vtavernier/ghsec/commit/07985815a8ca6c85129559eb85a72b3e7e3036cd)) - [@vtavernier](https://github.com/vtavernier)
- run cargo fmt - ([5c3d4de](https://github.com/vtavernier/ghsec/commit/5c3d4de207273d33b06b778c6b84f799e7c0610d)) - [@vtavernier](https://github.com/vtavernier)
- set repository metadata - ([ced47d0](https://github.com/vtavernier/ghsec/commit/ced47d0bfafbcd5279187b5289eca7c36c035927)) - [@vtavernier](https://github.com/vtavernier)
#### Refactoring
- move checks to dedicated enum variants - ([afdc9e2](https://github.com/vtavernier/ghsec/commit/afdc9e24fae84a87dcd511c31f9bd8c0ad0de199)) - [@vtavernier](https://github.com/vtavernier)

- - -

## [v0.1.0](https://github.com/vtavernier/ghsec/compare/a5ebcac1e753e61f29505b1b33e63c7e14a74eb3..v0.1.0) - 2023-12-26
#### Features
- initial release - ([a1b99fb](https://github.com/vtavernier/ghsec/commit/a1b99fb254f376545f80a045c8826f7efcb0de5b)) - [@vtavernier](https://github.com/vtavernier)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).