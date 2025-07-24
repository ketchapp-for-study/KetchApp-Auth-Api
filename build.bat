@echo off
setlocal

REM Set the project name (adjust if necessary to match your docker-compose.yml)
set PROJECT_NAME=ketchapp-auth

REM Compose file path
set COMPOSE_FILE=docker-compose.yml

echo Stopping and removing existing containers and volumes...
docker compose -f "%COMPOSE_FILE%" down -v

echo Starting the database container...
docker compose -f "%COMPOSE_FILE%" up -d db

REM Wait for Postgres to be ready (requires pg_isready in PATH, otherwise skip this block)
REM :wait_pg
REM pg_isready -h localhost -p 5432 -U postgres
REM if errorlevel 1 (
REM     echo Waiting for Postgres...
REM     timeout /t 1 >nul
REM     goto wait_pg
REM )

REM Run migrations (requires diesel CLI in PATH)
REM set DATABASE_URL=postgresql://postgres:password@localhost:5432/postgres
REM echo Running diesel migration run...
REM diesel migration run

echo Starting the API...
docker compose -f "%COMPOSE_FILE%" up -d auth-api

endlocal
