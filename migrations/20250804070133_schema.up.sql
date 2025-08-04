-- Add migration script here

CREATE TYPE user_type AS ENUM (
    'guest',
    'admin',
    'registered'
);

CREATE TYPE game_category AS ENUM (
    'warm_up',
    'casual',
    'spicy',
    'dangerous',
    'ladies',
    'boys'
);

CREATE TABLE "user" (
    "id" SERIAL PRIMARY KEY,
    "guest_id" INTEGER,
    "auth0_id" VARCHAR,
    "user_type" user_type NOT NULL DEFAULT 'guest',
    "last_active" TIMESTAMPTZ NOT NULL DEFAULT now(),
    "name" VARCHAR(100),
    "email" VARCHAR(150),
    "age" INTEGER
);

CREATE TABLE "quiz" (
    "id" SERIAL PRIMARY KEY,
    "name" VARCHAR(100) NOT NULL,
    "description" VARCHAR(150),
    "category" game_category NOT NULL DEFAULT 'casual',
    "iterations" INTEGER NOT NULL DEFAULT 0,
    "current_iterations" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE "question" (
    "id" SERIAL PRIMARY KEY,
    "quiz_id" INTEGER NOT NULL,
    "title" VARCHAR(200)
);

CREATE TABLE "spinner" (
    "id" SERIAL PRIMARY KEY,
    "host_id" INTEGER NOT NULL,
    "name" VARCHAR(100) NOT NULL,
    "description" VARCHAR(150),
    "category" game_category NOT NULL DEFAULT 'casual',
    "iterations" INTEGER NOT NULL DEFAULT 0,
    "current_iterations" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE "round" (
    "id" SERIAL PRIMARY KEY,
    "spinner_id" INTEGER NOT NULL,
    "host_id" INTEGER NOT NULL,
    "participants" INTEGER NOT NULL DEFAULT 0,
    "read_before" BOOLEAN NOT NULL,
    "title" VARCHAR(200)
);

CREATE TABLE "spinner_player" (
    "id" SERIAL PRIMARY KEY,
    "spinner_id" INTEGER NOT NULL,
    "user_id" INTEGER NOT NULL,
    "times_choosen" INTEGER NOT NULL DEFAULT 0
);

ALTER TABLE "question" ADD CONSTRAINT "quiz_question" FOREIGN KEY ("quiz_id") REFERENCES "quiz" ("id");
ALTER TABLE "round" ADD CONSTRAINT "spinner_round" FOREIGN KEY ("spinner_id") REFERENCES "spinner" ("id");
ALTER TABLE "spinner_player" ADD CONSTRAINT "spinner_player_fk" FOREIGN KEY ("spinner_id") REFERENCES "spinner" ("id");
ALTER TABLE "spinner_player" ADD CONSTRAINT "spinner_player_user_fk" FOREIGN KEY ("user_id") REFERENCES "user" ("id");

CREATE INDEX "idx_guest_user_id" ON "user" ("guest_id");
CREATE INDEX "idx_quiz_category" ON "quiz" ("category");
CREATE INDEX "idx_spinner_category" ON "spinner" ("category");