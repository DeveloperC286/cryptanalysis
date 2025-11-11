FROM rust:1.91.1-alpine3.21@sha256:ca42ef545d9c0c663f7dcb843705626e8eb09d150973109befb9244a091ffe8d
RUN rustup component add rustfmt

ENTRYPOINT ["cargo", "fmt", "--all", "--", "--config=group_imports=StdExternalCrate"]
