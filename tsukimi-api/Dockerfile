FROM rust:1.88-slim AS dev
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev
RUN cargo install watchexec-cli
WORKDIR /app

# RUN cargo build
CMD ["watchexec", \
    "--restart", \
    "--debounce", "500ms", \
    "--force-poll", "1s", \
    "--watch", "src", \
    "--watch", "Cargo.toml", \
    "--ignore", "target", \
    "--ignore", ".git", \
    "--", "cargo", "run"]
