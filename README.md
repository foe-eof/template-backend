# backend-template

The backend language is Rust. This repo contains the project skeleton for the backend part.

## IDE

The recommended IDE is vscode. If you have other preferences, you are responsible for setting up
your development environment.

## Rust Toolchain

Rust 2024 is almost released, so the toolchain is set to the beta channel to include all the
features we will have access to in 3 weeks. Furthermore, Rust 2024 will be released in stable
channel before the hackathon, so it would be great to have access to the features we are going to
work with.

## Cargo Workspace

The repo uses cargo workspace. The general rule is to use `crates/bin` for packages with binaries
and `cargo/lib` for library packages that are going to be used by the bins.

## Rust Stack

- Async Runtime: `tokio`
- Web Framework: `axum`
