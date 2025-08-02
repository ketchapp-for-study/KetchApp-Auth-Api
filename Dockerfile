# Stage 1: Build
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
LABEL maintainer="KetchAuthApi"
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq-dev ca-certificates postgresql-client sed && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/src/app/target/release/ketchapp-auth-api /app/ketchapp-auth-api
COPY config.toml .
COPY diesel.toml .
COPY private_key.pem .
COPY migrations ./migrations
EXPOSE 8080
CMD ["./ketchapp-auth-api"]
