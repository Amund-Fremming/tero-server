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

gitignore path:
    echo "\n{{path}}" >> .gitignore
    git rm --cached "{{path}}"
    git add .gitignore
    git commit -m "Removed cached file {{path}}"
    git push