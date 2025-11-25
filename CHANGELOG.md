# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
