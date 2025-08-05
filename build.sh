#!/bin/bash

# Set the project name (adjust if necessary to match your docker-compose.yml)
PROJECT_NAME="ketchapp-auth"

# Compose file path
COMPOSE_FILE="docker-compose.yml"

# Build step
echo "Building Docker image with Dockerfile..."
docker build -t ketchapp-auth-api .

# Stop and remove existing containers and volumes
echo "Stopping and removing existing containers and volumes..."
docker compose -f "$COMPOSE_FILE" down -v

# Start the database container
echo "Starting the database container..."
docker compose -f "$COMPOSE_FILE" up -d db

# Wait for Postgres to be ready
echo "Waiting for Postgres to be ready..."
start_time=$(date +%s)
timeout_seconds=30
until pg_isready -h localhost -p 5432 -U postgres; do
  current_time=$(date +%s)
  elapsed_time=$((current_time - start_time))
  if [ "$elapsed_time" -ge "$timeout_seconds" ]; then
    echo "Error: Postgres did not become ready within ${timeout_seconds} seconds. Exiting."
    exit 1
  fi
  echo "Waiting for Postgres..."
  sleep 1
done

# Run migrations on every startup
export DATABASE_URL=postgresql://postgres:password@localhost:5432/postgres
echo "Running diesel migration run..."
diesel migration run

# Start the authentication container directly (update service name if needed)
echo "Starting the API..."
docker compose -f "$COMPOSE_FILE" up -d auth-api
