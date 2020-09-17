-- Your SQL goes here
CREATE TABLE shortcuts (
    id INTEGER PRIMARY KEY NOT NULL,
    code TEXT(8) NOT NULL,
    url TEXT(512) NOT NULL,
    visits_count INTEGER NOT NULL DEFAULT 0
);
