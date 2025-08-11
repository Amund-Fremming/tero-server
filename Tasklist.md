# Tasklist

## Thoughts

- Implement wrapper objects for cache, enabling games to have questions and players
- Change from doing opåerations on user with auth0_id, or create a index on the field
- Consider changing from validating if a user is a guest from the header to a permission based validation
- Split the fn get_subject_and_permissins into two fn, SRP
- Rename subject mw to auth_mw or so?

## Tasklist

**Setup**
- [x] Init github
- [x] Update the old repos readme: - rewrite in rust
- [x] Setup dev env with docker compose
- [x] Setup basic api for health and health detailed
- [x] Setup tracing
- [x] Basic middleware request logger (may not need after some time, but good for debugging)
- [x] Map out all models
- [ ] Setup runtimes with .env files GITIGNORE, maybe also runtimmes for auth0, so we have separate triggers for different environments. Less pain to work with

**State**
- [x] Implement state with pg pool

**Error**
- [x] Implement descriptive error handling with internal logging not visible to the outside
- [x] Implement IntoResponse for all errors for the ServerError

**Auth0**
- [x] App (fe) application setup
- [x] API (be) setup
- [x] Add permissions

**User/Auth**
- [x] Add support for guest user and persistet user
- [x] Create middleware for injecting an extention for user
- [x] Post, put, delete
- [x] Put endpoint for updating last active
- [x] Auth0 webhook for users
- [x] Implement peristent storage for webhook api
- [x] Permissions extention
- [x] List all users (admin access)
- [x] Decode and validate tokens
- [ ] Permission checks for endpoints
- [ ] Maybe update endpoints to require user id for fetching users, targeting query on id, not auth0_id or guest_id. this also makes it possible for admins to query users 
- [ ] Update user sync from backend to auth0 (daily job/trigger)

**Cache**
- [ ] Implement a cache for Quiz 
- [ ] Implement a cache for Spinner 
- [ ] Implement a cache for Quiz search pages
- [ ] Implement a cache for Spinner search pages

**WebSocket**
- [ ] Add websocket support
- [ ] Add handling for sending, recieving different payloads and broadcasting
- [ ] Add handling for disconnects and connects

**Games**
- [ ] Implement core logic
- [ ] Only save game to disk when game is finished

**UniversalService**
- [ ] Pagination support
- [ ] Typed search for all games
- [ ] Universal join game

**Admin**
- [ ] Endponints for user history, how many active last week, last month and today
- [ ] Endpoints for fetching logs based on time or ceverity?

**Audit**
- [ ] Enums for action and ceverity
- [ ] Implement and SQL migration
- [ ] Add audit logs where neccesarry

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