clear:
    rm *.sql
dev file_paths:
    cargo run -- {{file_paths}}
pub-dry:
    cargo publish --dry-run
pub:
    cargo publish
