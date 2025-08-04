# Tasklist

## Tasklist

**Setup**
- [x] Init github
- [x] Update the old repos readme: - rewrite in rust
- [x] Setup dev env with docker compose
- [x] Setup basic api for health and health detailed
- [x] Setup tracing
- [ ] Basic middleware request logger (may not need after some time, but good for debugging)
- [ ] Setup runtimes and their tracing 
- [ ] Map out all models

**State**
- [ ] Implement state with pg pool

**Error**
- [ ] Implement descriptive error handling with internal logging not visible to the outside
- [ ] Implement IntoResponse for all errors for the ServerError

**User**
- [ ] Add support for guest user and persistet user
- [ ] Auth0 support for persistent user
- [ ] Create middleware for injecting an extention for user
- [ ] Post, patch, put, delete

**Cache**
- [ ] Implement a cache for games

**WebSocket**
- [ ] Add websocket support
- [ ] Add handling for sending, recieving different payloads and broadcasting
- [ ] Add handling for disconnects and connects

**Games**
- [ ] Implement core logic
- [ ] Only save game to disk when game is finished

**Admin**
- [ ] Endponints for user history, how many active last week, last month and today

---

## Modelling

- user
    - id (PK)
    - pseudo_id
    - auth0_id (nullable)
    - user_type (enum: admin, guest, persistent)
    - last_active 
    - last_updated (nullable)
    - name (nullable)
    - email (nullable)
    - age (nullable)

- quiz
    - id (PK)
    - name
    - description
    - iterations
    - current_iteration

- question
    - id (PK)
    - quiz_id (FK)
    - body

- spinner
    - id (PK)
    - name
    - description
    - iterations
    - current_iteration

- round
    - id (PK) 
    - host_id
    - participants
    - read_before (flag)
    - body

- spinner_user
    - id (PK)
    - spinner_id (FK)
    - user_id (FK)
    - times_choosen

???????????????
- user_game_relation
    - id (PK)
    - user_id (FK)
    - game_id (FK)
    - game_type (enum: spinner, quiz, ...)
    - saved

---

## Architecure

```md
/tero_backend
    /src
        /spinner
            service.rs
            models.rs
            game.rs
            db.rs
            handlers.rs
            routes.rs
            mod.rs
        /quiz
            service.rs
            models.rs
            game.rs
            db.rs
            handlers.rs
            routes.rs
            mod.rs
        /auth (user)
            mw.rs
            service.rs
            models.rs
            db.rs
            handlers.rs
            routes.rs
            mw.rs
            mod.rs
        /error
            server_error.rs
            api_error.rs
            mod.rs
        /state
            cache.rs
            app_state.rs
            mod.rs
        /common
            models.rs
            mod.rs
        /admin
            handlers.rs
            service.rs
            db.rs
            mod.rs
        /ws
            handlers.rs
            parsers.rs
            mod.rs
        /common
            universal_game_service.rs
            mod.rs
        mw.rs
        main.rs
```