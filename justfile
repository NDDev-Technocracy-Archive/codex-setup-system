set shell := ["bash", "-euo", "pipefail", "-c"]
set positional-arguments

# List the maintained source and native commands.
default:
    @just --list

# Run readonly checks without compiling or launching the provider.
check:
    just --fmt --check
    cargo fmt --all --check
    actionlint
    zizmor --offline --no-progress .github

# Preserve the native Clippy, build, package self-check and all-target tests.
test: check
    python tools/with_libclang.py cargo clippy --locked --all-targets -- -D warnings
    python tools/with_libclang.py cargo build --locked --all-targets
    python tools/with_libclang.py python tools/build_crates_io.py --self-check
    python tools/with_libclang.py cargo test --locked --all-targets

# Build the release artifact with the same isolated libclang provider.
build:
    python tools/with_libclang.py cargo build --locked --release

# Retain all three existing Rust supply-chain gates.
audit-dependencies:
    cargo deny --all-features check bans licenses advisories sources
    cargo audit
    cargo-machete

# Retain the complete recursive lockfile vulnerability scan.
audit-lockfiles:
    osv-scanner scan source --recursive .

# Scan the exact PR range or complete history; no provider workload runs.
audit-source scope="--all": check
    betterleaks git --no-banner --redact "--log-opts=$1" .

# Run the full local/native gate; platform and artifact boundary jobs stay separate.
verify: check test build audit-dependencies audit-lockfiles audit-source

# Format the Rust and command source.
fix:
    just --fmt
    cargo fmt --all
