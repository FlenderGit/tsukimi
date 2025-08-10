
FROM rust:1.88-slim AS dev
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev
RUN cargo install trunk
WORKDIR /app

CMD ["trunk", "serve", \
    "--port", "8080", \
    "--address", "0.0.0.0"]
