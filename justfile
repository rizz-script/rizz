dev *ARGS:
    cargo run --bin rizz -- {{ ARGS }}

test-all:
    cargo run --bin rizz -- run examples/fs_shell_smoke.rizz
    cargo run --bin rizz -- run examples/hello.rizz
    cargo run --bin rizz -- run examples/github_profile.rizz github
    cargo run --bin rizz -- run examples/import_smoke.rizz
    cargo run --bin rizz -- run examples/grep.rizz README.md "rizz"
    cargo run --bin rizz -- run examples/js_keywords_smoke.rizz

build:
    cargo build --release --bin rizz

playground:
    cd apps/rizz-playground && bun run build:wasm && bun run dev

check:
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt --all -- --check
    cargo test --all --all-features

docs:
    cd docs && bun run dev