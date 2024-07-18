# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## [v0.5.0](https://github.com/alixinne/ghsec/compare/v0.4.0..v0.5.0) - 2023-12-27
#### Bug Fixes
- expose clickable links as tracing fields - ([18678f1](https://github.com/alixinne/ghsec/commit/18678f191cd3d8118123b192677704323a958b5b)) - [@alixinne](https://github.com/alixinne)
#### Continuous Integration
- publish to crates.io - ([56cfc7b](https://github.com/alixinne/ghsec/commit/56cfc7bd37f4f66f1d883c7e45961cbc5063240b)) - [@alixinne](https://github.com/alixinne)
#### Features
- implement basic branch_protections check - ([ed8ac90](https://github.com/alixinne/ghsec/commit/ed8ac908ad89b6550539353ec270e83fa54b38a5)) - [@alixinne](https://github.com/alixinne)

- - -

## [v0.4.0](https://github.com/alixinne/ghsec/compare/v0.3.0..v0.4.0) - 2023-12-27
#### Continuous Integration
- use cog changelog in releases instead of --generate-notes - ([bb4225a](https://github.com/alixinne/ghsec/commit/bb4225a6c655673d9e8fdb0baabe52f002605df1)) - [@alixinne](https://github.com/alixinne)
#### Documentation
- **(code_review_limits)** fix typo - ([f1d5e8a](https://github.com/alixinne/ghsec/commit/f1d5e8af40c61021f8dc1721646f59682d767aef)) - [@alixinne](https://github.com/alixinne)
- fix list in readme - ([9addb63](https://github.com/alixinne/ghsec/commit/9addb634cabc2cfd92d246bd29cf8f89aa33690d)) - [@alixinne](https://github.com/alixinne)
#### Features
- add fork_pull_request_workflow check - ([cf98b35](https://github.com/alixinne/ghsec/commit/cf98b35491bbb90572a63f9235d7c2ae456cfcae)) - [@alixinne](https://github.com/alixinne)

- - -

## [v0.3.0](https://github.com/alixinne/ghsec/compare/v0.2.0..v0.3.0) - 2023-12-27
#### Documentation
- **(checks)** add documentation for current checks - ([bcc10f7](https://github.com/alixinne/ghsec/commit/bcc10f7fa5b6aaaad8cc0a4dfe70b437a7f03b33)) - [@alixinne](https://github.com/alixinne)
- fix links to supported checks - ([1cb20d1](https://github.com/alixinne/ghsec/commit/1cb20d1742204e32b3dfda8823188671329e014e)) - [@alixinne](https://github.com/alixinne)
- add LICENSE - ([4eab024](https://github.com/alixinne/ghsec/commit/4eab024c9fa9f7607ab342f3af3dcc4c5899b3a9)) - [@alixinne](https://github.com/alixinne)
#### Features
- **(repository_secrets)** add secret name regex for triggering warnings - ([f199330](https://github.com/alixinne/ghsec/commit/f1993301d26a606238d6482ebb4c6d8df69869bd)) - [@alixinne](https://github.com/alixinne)
- add code_review_limits check - ([d20ffc3](https://github.com/alixinne/ghsec/commit/d20ffc3d0865e9336549d881d38b513428a33743)) - [@alixinne](https://github.com/alixinne)
- filter target repositories using glob - ([33192c7](https://github.com/alixinne/ghsec/commit/33192c75f195c70ab76419c57468d29046410dbc)) - [@alixinne](https://github.com/alixinne)

- - -

## [v0.2.0](https://github.com/alixinne/ghsec/compare/v0.1.0..v0.2.0) - 2023-12-26
#### Bug Fixes
- update logging and fix behavior - ([7ca9e04](https://github.com/alixinne/ghsec/commit/7ca9e041505ff1a78917ecd488031647dad5b24c)) - [@alixinne](https://github.com/alixinne)
#### Build system
- set version in Cargo.toml on release - ([e581820](https://github.com/alixinne/ghsec/commit/e581820b78a6201ac7064f0555fa5d7b18c8a734)) - [@alixinne](https://github.com/alixinne)
- generate readme on release - ([aafd22c](https://github.com/alixinne/ghsec/commit/aafd22cb4830ca9c86b9892280aa4a0d90b75e4d)) - [@alixinne](https://github.com/alixinne)
#### Continuous Integration
- enable github pages build - ([f201c1b](https://github.com/alixinne/ghsec/commit/f201c1b02df6378e8311a965aec2ce2b29a68bef)) - [@alixinne](https://github.com/alixinne)
- add GITHUB_TOKEN to environment - ([0b736c4](https://github.com/alixinne/ghsec/commit/0b736c40eedaaca8b5b100c2dacd8709113db645)) - [@alixinne](https://github.com/alixinne)
#### Documentation
- add main module documentation - ([c89ae3c](https://github.com/alixinne/ghsec/commit/c89ae3cb91bf8638b1868c0e0a83c4cc3193af0f)) - [@alixinne](https://github.com/alixinne)
#### Features
- support cargo-binstall - ([14a6169](https://github.com/alixinne/ghsec/commit/14a6169622afffad0ee7d54a3c0ed6be3e48e104)) - [@alixinne](https://github.com/alixinne)
- add json and debug logging options - ([ac90704](https://github.com/alixinne/ghsec/commit/ac907040c75b16719b92e27326ac44ebe6a7b31a)) - [@alixinne](https://github.com/alixinne)
- allow configuring checks to run - ([045b193](https://github.com/alixinne/ghsec/commit/045b193f4bb1f056f0f5b3e6231461b4452e51c2)) - [@alixinne](https://github.com/alixinne)
#### Miscellaneous Chores
- **(gitignore)** ignore built binaries - ([0798581](https://github.com/alixinne/ghsec/commit/07985815a8ca6c85129559eb85a72b3e7e3036cd)) - [@alixinne](https://github.com/alixinne)
- run cargo fmt - ([5c3d4de](https://github.com/alixinne/ghsec/commit/5c3d4de207273d33b06b778c6b84f799e7c0610d)) - [@alixinne](https://github.com/alixinne)
- set repository metadata - ([ced47d0](https://github.com/alixinne/ghsec/commit/ced47d0bfafbcd5279187b5289eca7c36c035927)) - [@alixinne](https://github.com/alixinne)
#### Refactoring
- move checks to dedicated enum variants - ([afdc9e2](https://github.com/alixinne/ghsec/commit/afdc9e24fae84a87dcd511c31f9bd8c0ad0de199)) - [@alixinne](https://github.com/alixinne)

- - -

## [v0.1.0](https://github.com/alixinne/ghsec/compare/a5ebcac1e753e61f29505b1b33e63c7e14a74eb3..v0.1.0) - 2023-12-26
#### Features
- initial release - ([a1b99fb](https://github.com/alixinne/ghsec/commit/a1b99fb254f376545f80a045c8826f7efcb0de5b)) - [@alixinne](https://github.com/alixinne)

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).