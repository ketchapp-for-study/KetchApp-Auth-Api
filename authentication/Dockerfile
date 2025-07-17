# Stage 1: Build
FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/app
LABEL maintainer="KetchAuthApi"
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/authentication /app/authentication
COPY config.toml .
EXPOSE 8080
CMD ["./authentication"]