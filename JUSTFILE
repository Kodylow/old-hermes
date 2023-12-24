db:
    docker compose up -d

reset db:
    docker compose down -v && docker compose up -d

migrate:
    cargo sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5432/proteus"

seed:
    psql "postgres://postgres:postgres@127.0.0.1:5432/proteus" -a -f sql/seed.sql
