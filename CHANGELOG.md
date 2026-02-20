# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.11](https://github.com/timvw/bose/compare/v0.2.10...v0.2.11) - 2026-02-20

### Other

- *(deps)* update rust crate clap to v4.5.60 ([#48](https://github.com/timvw/bose/pull/48))
- *(deps)* update rust crate anyhow to v1.0.102 ([#49](https://github.com/timvw/bose/pull/49))
- *(deps)* update rust crate clap to v4.5.59 ([#47](https://github.com/timvw/bose/pull/47))
- *(deps)* update rust crate clap to v4.5.58 ([#46](https://github.com/timvw/bose/pull/46))
- *(deps)* update rust crate clap to v4.5.57 ([#43](https://github.com/timvw/bose/pull/43))
- Bump bytes from 1.11.0 to 1.11.1 ([#44](https://github.com/timvw/bose/pull/44))
- *(deps)* update rust crate anyhow to v1.0.101
- *(deps)* update rust crate clap to v4.5.55
- Merge pull request #41 from timvw/renovate/tokio-1.x-lockfile
- *(deps)* update rust crate tokio to v1.49.0
- *(deps)* update github artifact actions

## [0.2.10](https://github.com/timvw/bose/compare/v0.2.9...v0.2.10) - 2025-11-25

### Other

- Let release-plz tag only; GitHub release handled by Release workflow

## [0.2.9](https://github.com/timvw/bose/compare/v0.2.8...v0.2.9) - 2025-11-25

### Fixed

- windows copy syntax
- upload artifacts from dist
- correct env syntax in release workflow

### Other

- Remove registry manifest flag for release step
- release v0.2.8
- Label release-plz PRs automatically
- Split release-plz: PR on main pushes, tag only on merged release PRs
- Trigger release tagging only on merged release-labeled PRs
- Remove release process section from README
- release v0.2.7
- Fix secrets context in release-plz workflow
- Use only GitHub App token for release-plz
- Use GitHub App token for release-plz PRs
- Prefer timvw-ci-bot token for release-plz
- Use RELEASE_PLZ_TOKEN for release-plz PRs
- release v0.2.5
- release v0.2.3
- Use current manifest as registry baseline for release-plz
- Bump Cargo.toml to 0.2.3
- Bump version to 0.2.3
- Remove unsupported registry field from release-plz config
- Run release-plz with registry manifest from latest tag
- Stop release-plz from hitting crates.io
- Fix release-plz config and align workflow
- Set GITHUB_TOKEN for release-plz jobs
- Add release-plz workflow for release PR and tags
- Restore minimal release validation workflow
- Fix heredoc indentation in release workflow
- Resolve release tag safely and pass through job outputs
- Fix release tag resolution for workflow_dispatch
- Guard all release jobs to tag/dispatch triggers
- Allow push on main to pass release workflow
- Skip release workflow on non-tag pushes
- Guard release workflow to tags/dispatch
- Add release tag/version validation
- Restore settings configuration
- Add settings to delete merged branches
- Remove outdated README sections
- Require CI check on main
- Add CI workflow for Rust checks
- Enable auto-delete merged branches via settings app
- *(deps)* update rust crate bose_soundtouch to v2.1.2
- *(deps)* update actions/checkout action to v6
- Merge pull request #24 from timvw/renovate/major-github-artifact-actions
- bump to 0.2.2
- add release workflow
- mention homebrew tap install
- *(deps)* update rust crate clap to v4.5.53
- bump version to 0.2.0
- Use bose_soundtouch library for CLI
- update versions
- Merge pull request #11 from timvw/renovate/serde-monorepo
- Merge pull request #13 from timvw/renovate/reqwest-0.x-lockfile
- Merge pull request #14 from timvw/renovate/clap-4.x-lockfile
- Update Rust crate anyhow to v1.0.83
- Merge pull request #7 from timvw/renovate/anyhow-1.x
- Update Rust crate reqwest to 0.12
- Update Rust crate serde to 1.0.198
- Update Rust crate clap to 4.4.18
- Add renovate.json
- added bose soundtouch api
- inital commit
- Initial commit

## [0.2.8](https://github.com/timvw/bose/compare/v0.2.7...v0.2.8) - 2025-11-25

### Fixed

- windows copy syntax
- upload artifacts from dist
- correct env syntax in release workflow

### Other

- Label release-plz PRs automatically
- Split release-plz: PR on main pushes, tag only on merged release PRs
- Trigger release tagging only on merged release-labeled PRs
- Remove release process section from README
- release v0.2.7
- Fix secrets context in release-plz workflow
- Use only GitHub App token for release-plz
- Use GitHub App token for release-plz PRs
- Prefer timvw-ci-bot token for release-plz
- Use RELEASE_PLZ_TOKEN for release-plz PRs
- release v0.2.5
- release v0.2.3
- Use current manifest as registry baseline for release-plz
- Bump Cargo.toml to 0.2.3
- Bump version to 0.2.3
- Remove unsupported registry field from release-plz config
- Run release-plz with registry manifest from latest tag
- Stop release-plz from hitting crates.io
- Fix release-plz config and align workflow
- Set GITHUB_TOKEN for release-plz jobs
- Add release-plz workflow for release PR and tags
- Restore minimal release validation workflow
- Fix heredoc indentation in release workflow
- Resolve release tag safely and pass through job outputs
- Fix release tag resolution for workflow_dispatch
- Guard all release jobs to tag/dispatch triggers
- Allow push on main to pass release workflow
- Skip release workflow on non-tag pushes
- Guard release workflow to tags/dispatch
- Add release tag/version validation
- Restore settings configuration
- Add settings to delete merged branches
- Remove outdated README sections
- Require CI check on main
- Add CI workflow for Rust checks
- Enable auto-delete merged branches via settings app
- *(deps)* update rust crate bose_soundtouch to v2.1.2
- *(deps)* update actions/checkout action to v6
- Merge pull request #24 from timvw/renovate/major-github-artifact-actions
- bump to 0.2.2
- add release workflow
- mention homebrew tap install
- *(deps)* update rust crate clap to v4.5.53
- bump version to 0.2.0
- Use bose_soundtouch library for CLI
- update versions
- Merge pull request #11 from timvw/renovate/serde-monorepo
- Merge pull request #13 from timvw/renovate/reqwest-0.x-lockfile
- Merge pull request #14 from timvw/renovate/clap-4.x-lockfile
- Update Rust crate anyhow to v1.0.83
- Merge pull request #7 from timvw/renovate/anyhow-1.x
- Update Rust crate reqwest to 0.12
- Update Rust crate serde to 1.0.198
- Update Rust crate clap to 4.4.18
- Add renovate.json
- added bose soundtouch api
- inital commit
- Initial commit

## [0.2.7](https://github.com/timvw/bose/compare/v0.2.6...v0.2.7) - 2025-11-25

### Fixed

- windows copy syntax
- upload artifacts from dist
- correct env syntax in release workflow

### Other

- Fix secrets context in release-plz workflow
- Use only GitHub App token for release-plz
- Use GitHub App token for release-plz PRs
- Prefer timvw-ci-bot token for release-plz
- Use RELEASE_PLZ_TOKEN for release-plz PRs
- release v0.2.5
- release v0.2.3
- Use current manifest as registry baseline for release-plz
- Bump Cargo.toml to 0.2.3
- Bump version to 0.2.3
- Remove unsupported registry field from release-plz config
- Run release-plz with registry manifest from latest tag
- Stop release-plz from hitting crates.io
- Fix release-plz config and align workflow
- Set GITHUB_TOKEN for release-plz jobs
- Add release-plz workflow for release PR and tags
- Restore minimal release validation workflow
- Fix heredoc indentation in release workflow
- Resolve release tag safely and pass through job outputs
- Fix release tag resolution for workflow_dispatch
- Guard all release jobs to tag/dispatch triggers
- Allow push on main to pass release workflow
- Skip release workflow on non-tag pushes
- Guard release workflow to tags/dispatch
- Add release tag/version validation
- Restore settings configuration
- Add settings to delete merged branches
- Remove outdated README sections
- Require CI check on main
- Add CI workflow for Rust checks
- Enable auto-delete merged branches via settings app
- *(deps)* update rust crate bose_soundtouch to v2.1.2
- *(deps)* update actions/checkout action to v6
- Merge pull request #24 from timvw/renovate/major-github-artifact-actions
- bump to 0.2.2
- add release workflow
- mention homebrew tap install
- *(deps)* update rust crate clap to v4.5.53
- bump version to 0.2.0
- Use bose_soundtouch library for CLI
- update versions
- Merge pull request #11 from timvw/renovate/serde-monorepo
- Merge pull request #13 from timvw/renovate/reqwest-0.x-lockfile
- Merge pull request #14 from timvw/renovate/clap-4.x-lockfile
- Update Rust crate anyhow to v1.0.83
- Merge pull request #7 from timvw/renovate/anyhow-1.x
- Update Rust crate reqwest to 0.12
- Update Rust crate serde to 1.0.198
- Update Rust crate clap to 4.4.18
- Add renovate.json
- added bose soundtouch api
- inital commit
- Initial commit

## [0.2.6](https://github.com/timvw/bose/compare/v0.2.5...v0.2.6) - 2025-11-25

### Other

- Prefer timvw-ci-bot token for release-plz
- Use RELEASE_PLZ_TOKEN for release-plz PRs

## [0.2.5](https://github.com/timvw/bose/compare/v0.2.4...v0.2.5) - 2025-11-25

### Fixed

- windows copy syntax
- upload artifacts from dist
- correct env syntax in release workflow

### Other

- release v0.2.3
- Use current manifest as registry baseline for release-plz
- Bump Cargo.toml to 0.2.3
- Bump version to 0.2.3
- Remove unsupported registry field from release-plz config
- Run release-plz with registry manifest from latest tag
- Stop release-plz from hitting crates.io
- Fix release-plz config and align workflow
- Set GITHUB_TOKEN for release-plz jobs
- Add release-plz workflow for release PR and tags
- Restore minimal release validation workflow
- Fix heredoc indentation in release workflow
- Resolve release tag safely and pass through job outputs
- Fix release tag resolution for workflow_dispatch
- Guard all release jobs to tag/dispatch triggers
- Allow push on main to pass release workflow
- Skip release workflow on non-tag pushes
- Guard release workflow to tags/dispatch
- Add release tag/version validation
- Restore settings configuration
- Add settings to delete merged branches
- Remove outdated README sections
- Require CI check on main
- Add CI workflow for Rust checks
- Enable auto-delete merged branches via settings app
- *(deps)* update rust crate bose_soundtouch to v2.1.2
- *(deps)* update actions/checkout action to v6
- Merge pull request #24 from timvw/renovate/major-github-artifact-actions
- bump to 0.2.2
- add release workflow
- mention homebrew tap install
- *(deps)* update rust crate clap to v4.5.53
- bump version to 0.2.0
- Use bose_soundtouch library for CLI
- update versions
- Merge pull request #11 from timvw/renovate/serde-monorepo
- Merge pull request #13 from timvw/renovate/reqwest-0.x-lockfile
- Merge pull request #14 from timvw/renovate/clap-4.x-lockfile
- Update Rust crate anyhow to v1.0.83
- Merge pull request #7 from timvw/renovate/anyhow-1.x
- Update Rust crate reqwest to 0.12
- Update Rust crate serde to 1.0.198
- Update Rust crate clap to 4.4.18
- Add renovate.json
- added bose soundtouch api
- inital commit
- Initial commit

## [0.2.4](https://github.com/timvw/bose/compare/v0.2.3...v0.2.4) - 2025-11-25

### Other

- Use current manifest as registry baseline for release-plz
- Bump Cargo.toml to 0.2.3
- Bump version to 0.2.3
- Remove unsupported registry field from release-plz config
- Run release-plz with registry manifest from latest tag
- Stop release-plz from hitting crates.io
- Fix release-plz config and align workflow
- Set GITHUB_TOKEN for release-plz jobs
- Add release-plz workflow for release PR and tags
- Restore minimal release validation workflow
- Fix heredoc indentation in release workflow
- Resolve release tag safely and pass through job outputs
- Fix release tag resolution for workflow_dispatch
- Guard all release jobs to tag/dispatch triggers
- Allow push on main to pass release workflow
- Skip release workflow on non-tag pushes
- Guard release workflow to tags/dispatch
- Add release tag/version validation
- Restore settings configuration
- Add settings to delete merged branches
