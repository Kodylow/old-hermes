db:
    docker compose up -d

migrate:
    cargo sqlx migrate run --database-url "postgres://postgres:postgres@localhost:5432/fedimint_lnurl"
