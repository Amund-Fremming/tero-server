push msg:
    git add .
    git commit -m "{{msg}}"
    git push

start-all:
    docker compose up -d
    cargo run

nuke-start:
    docker compose down -v
    docker compose up -d
    sqlx migrate run