#!/bin/sh
set -e

export DATABASE_URL=${DATABASE_URL:-postgresql://postgres:password@ketchapp-auth-db:5432/postgres}

# Wait for Postgres to be ready
until pg_isready -h ketchapp-auth-db -U postgres; do
  echo "Waiting for Postgres..."
  sleep 1
done

cd /app

# Run migrations on every startup
diesel migration run

exec "$@"
