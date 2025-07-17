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
RUN apt-get update && apt-get install -y libpq-dev ca-certificates postgresql-client && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel
COPY --from=builder /usr/src/app/target/release/authentication /app/authentication
COPY config.toml .
COPY diesel.toml .
COPY migrations ./migrations
COPY entrypoint.sh .
RUN chmod +x entrypoint.sh
EXPOSE 8080
CMD ["./entrypoint.sh", "./authentication"]