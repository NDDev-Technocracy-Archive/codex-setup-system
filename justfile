set shell := ["bash", "-euo", "pipefail", "-c"]
set positional-arguments

# List the source, native and security commands.
default:
    @just --list

# Check source formatting and workflows without building a provider.
check:
    just --fmt --check
    cargo fmt --all --check
    actionlint
    zizmor --offline --no-progress --min-severity low .github

# Retain native Clippy, all-target builds/tests and package self-checks.
test:
    python tools/with_libclang.py cargo clippy --locked --all-targets -- -D warnings
    python tools/with_libclang.py cargo build --locked --all-targets
    python tools/with_libclang.py python tools/build_crates_io.py --self-check
    python tools/with_libclang.py cargo test --locked --all-targets

# Check all targets with the explicitly selected minimum Rust toolchain.
test-msrv toolchain:
    RUSTUP_TOOLCHAIN="$1" python tools/with_libclang.py cargo check --locked --all-targets

# Build the release provider through its existing libclang boundary.
build:
    python tools/with_libclang.py cargo build --locked --release

# Retain the three fail-closed Rust supply-chain checks.
audit-dependencies:
    cargo deny --all-features check bans licenses advisories sources
    cargo audit
    cargo-machete

# Retain the full recursive lockfile scan.
audit-lockfiles:
    osv-scanner scan source --recursive .

# Retain the separate complete-history secret scan.
audit-secrets:
    betterleaks git --no-banner --redact --log-opts=--all .

# Preserve the online regular-persona, low-severity SARIF publication instrument.
audit-workflows-sarif:
    zizmor --persona regular --min-severity low --format sarif . > zizmor.sarif

# Run the full local gate; platform and import-table jobs remain separate.
verify: check test build audit-dependencies audit-lockfiles audit-secrets

# Format the Rust and command source.
fix:
    just --fmt
    cargo fmt --all
