set dotenv-load := true

run:
    cargo run

db:
    docker-compose up -d

reset db:
    docker-compose down -v && docker-compose up -d && just migrate && just seed

migrate:
    cargo sqlx migrate run --database-url $DATABASE_URL

seed:
    psql $DATABASE_URL -a -f sql/seed.sql
