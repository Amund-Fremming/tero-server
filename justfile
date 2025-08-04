push msg:
    git add .
    git commit -m "{{msg}}"
    git push

hard-start:
    docker compose up -d
    cargo run

nuke:
    docker compose down -v
    docker compose up -d
    sqlx migrate run
