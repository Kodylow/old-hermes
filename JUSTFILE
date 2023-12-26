run:
    cargo run

db:
    docker compose up -d

reset db:
    docker compose down -v && docker compose up -d && just migrate && just seed

migrate:
    cargo sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5432/hermes"

seed:
    psql "postgres://postgres:postgres@127.0.0.1:5432/hermes" -a -f sql/seed.sql
