# Dependency Management

_Last updated: 2025-04-18_

## Overview

The Ordo project maintains a strict policy of using the latest stable versions of all dependencies to ensure security, performance, and access to the newest features. This document outlines our approach to dependency management and version requirements.

## Version Requirements

### Core Framework Versions

| Component | Minimum Version | Notes |
|-----------|----------------|-------|
| Rust | 1.76.0+ | Latest stable as of April 2025 |
| Tauri | 2.5.0+ | Latest stable release |
| Leptos | 0.8.0+ | Latest stable release |
| SQLite | 3.44.0+ | Latest stable release |
| Redb | 1.3.0+ | Latest stable release |

### Key Dependencies

The project uses the latest versions of these critical dependencies, the latest versions should be checked often to ensure they are up-to-date:

```toml
# Core dependencies
tokio = { version = "1.28", features = ["full"] }
axum = "0.7"
sqlx = { version = "0.8.3", features = ["runtime-tokio-rustls", "sqlite", "chrono", "macros"] }
tauri = "2.5"
leptos = "0.8"

# UI libraries
daisyui = "4.0"
plotly = "0.8"
tanstack-table = "0.3"

# Security
argon2 = "0.5.2"
jsonwebtoken = "9.3.1"
secrecy = "0.8"
password-hash = "0.5"
rand_core = "0.6"

# Background processing
background_jobs = "0.15"
tokio-beat = "0.3"

# Storage
redb = "1.3.0"
bincode = "1.3.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Utilities
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.16.0", features = ["v4", "serde"] }
tracing = "0.1.41"
```

## Version Policy

### Always Latest Stable

- **Use Latest Versions**: Always use the latest stable versions of all dependencies
- **Regular Updates**: Dependencies are updated on a bi-weekly schedule
- **Security Patches**: Security-related updates are applied immediately
- **Breaking Changes**: Major version upgrades are evaluated for breaking changes before implementation

### Version Pinning

In rare cases, specific versions may be pinned for stability:

```toml
# Example of pinned versions for critical components
redb = { version = "=1.3.0", default-features = false, features = ["logging"] }
ed25519-dalek = "=2.0.0"
```

Pinned versions should include a comment explaining why the version is pinned and when it should be reconsidered for updating.

## Dependency Auditing

The project uses several tools to ensure dependency quality and security:

1. **cargo-audit**: Regular security audits of all dependencies
2. **cargo-outdated**: Identifies outdated dependencies
3. **cargo-deny**: Enforces license and vulnerability policies
4. **GitHub Dependabot**: Automated dependency update PRs

## Windows Compatibility

Special attention is paid to Windows compatibility for all dependencies:

```toml
[target.'cfg(windows)'.dependencies]
background_jobs = { version = "0.15", features = ["windows-optimized"] }
tokio = { version = "1.28", features = ["rt-multi-thread", "time"] }
```

## Dependency Selection Criteria

When adding new dependencies, the following criteria are considered:

1. **Maintenance Status**: Active development and maintenance
2. **Community Support**: Widely used in the Rust ecosystem
3. **Documentation Quality**: Well-documented API and examples
4. **Performance**: Minimal overhead and resource usage
5. **License Compatibility**: Compatible with the project's license
6. **Security History**: No significant security issues

## Updating Dependencies

To update dependencies:

1. Run `cargo update` to update to the latest compatible versions
2. Run `cargo outdated` to identify dependencies that require major version updates
3. Test thoroughly after updates, especially after major version changes
4. Document any breaking changes and required code modifications

## Conclusion

By maintaining the latest versions of all dependencies, the Ordo project ensures security, performance, and access to the newest features. This approach helps prevent technical debt and ensures the project remains maintainable and secure over time.
