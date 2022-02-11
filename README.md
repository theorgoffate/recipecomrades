# Recipe Comrades

Recipe Comrades! is a GraphQL API to notate and share recipes with family, friends, and comrades.

## Requirements

1. [Rust 2021 Edition](https://rust-lang.org)
1. [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. [PostgreSQL](https://www.postgresql.org/)
1. [Insomnia](https://insomnia.rest/download) (optional for running GraphQL queries)

### Notes on Rust and Cargo

The easiest way to install Rust and Cargo in one go is to use [Rustup](https://rustup.rs).

### Notes on PostgreSQL

We use `.env` for storing our OS environment variables (e.g. database credentials). There is an example of the file in the root named `.env-example`. The quickest way to get up an running is to copy `.env-example` to `.env` and edit the credentials to match your PostgreSQL credentials.

## Building and Running

We use Cargo to handle all development build and run tasks:

- To build use `cargo build` (which will place the binary in the `target/debug` by default)
- To run use `cargo run`
