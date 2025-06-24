# ---- Build stage ----
FROM rust:1.87 AS builder
WORKDIR /app

# Copy only what's needed
COPY server /app/server
COPY ../protocol /app/protocol
COPY server/Server.toml /app/server/Server.toml

WORKDIR /app/server
RUN cargo build --bin server --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/server/Server.toml /Server.toml
COPY --from=builder /app/server/target/release/server /bin/server
CMD ["/bin/server"]
