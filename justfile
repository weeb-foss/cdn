alias mu := migrate-up
alias md := migrate-down
alias sp := sqlx-prepare

default:
    just --list

migrate-up:
    cargo sqlx migrate run

migrate-down:
    for _ in migrations/*.down.sql; do cargo sqlx migrate revert; done

sqlx-prepare:
    cargo sqlx prepare --workspace

rebuild-db:
    @echo -e "\x1b[1;33mcargo sqlx database drop\x1b[0m"; cargo sqlx database drop || echo "No Database"
    @echo -e "\x1b[1;33mcargo sqlx database create\x1b[0m"; cargo sqlx database create || exit 1
    @just mu

format:
    rustfmt --edition 2024 --config-path rustfmt.toml $(find . -name '*.rs')
