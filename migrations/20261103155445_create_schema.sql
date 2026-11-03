CREATE TABLE users (
    id         INTEGER PRIMARY KEY,
    uuid       TEXT    NOT NULL UNIQUE,
    name       TEXT    NOT NULL,
    created_at TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at TEXT
) STRICT;

CREATE TABLE books (
    id              INTEGER PRIMARY KEY,
    uuid            TEXT    NOT NULL UNIQUE,
    title           TEXT    NOT NULL,
    description     TEXT,
    openlibrary_key TEXT    UNIQUE,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT
) STRICT;

CREATE TABLE authors (
    id              INTEGER PRIMARY KEY,
    uuid            TEXT    NOT NULL UNIQUE,
    name            TEXT    NOT NULL,
    openlibrary_key TEXT    UNIQUE,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT
) STRICT;

CREATE TABLE genres (
    id              INTEGER PRIMARY KEY,
    uuid            TEXT    NOT NULL UNIQUE,
    name            TEXT    NOT NULL UNIQUE,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT
) STRICT;

CREATE TABLE editions (
    id              INTEGER PRIMARY KEY,
    uuid            TEXT    NOT NULL UNIQUE,
    book_id         INTEGER NOT NULL REFERENCES books(id) ON DELETE RESTRICT,
    published_year  INTEGER,
    cover_path      TEXT,
    language        TEXT,
    isbn13          TEXT    UNIQUE,
    publisher       TEXT,
    page_count      INTEGER,
    openlibrary_key TEXT    UNIQUE,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT
) STRICT;

CREATE TABLE book_authors (
    book_id   INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    author_id INTEGER NOT NULL REFERENCES authors(id) ON DELETE RESTRICT,
    position  INTEGER NOT NULL,
    PRIMARY KEY (book_id, author_id),
    UNIQUE (book_id, position)
) STRICT;

CREATE TABLE book_genres (
    book_id  INTEGER NOT NULL REFERENCES books(id) ON DELETE CASCADE,
    genre_id INTEGER NOT NULL REFERENCES genres(id) ON DELETE RESTRICT,
    PRIMARY KEY (book_id, genre_id)
) STRICT;

CREATE TABLE readings(
    id              INTEGER PRIMARY KEY,
    uuid            TEXT    NOT NULL UNIQUE,
    user_id         INTEGER NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    edition_id      INTEGER NOT NULL REFERENCES editions(id) ON DELETE RESTRICT,
    status          TEXT    NOT NULL DEFAULT 'pending'
                    CHECK (status IN ('pending', 'active', 'completed', 'dropped')),
    rating          INTEGER CHECK (rating BETWEEN 0 AND 10),
    review          TEXT,
    started_at      TEXT,
    finished_at     TEXT,
    created_at      TEXT    NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now')),
    updated_at      TEXT,
    CHECK (finished_at IS NULL OR started_at IS NULL OR finished_at >= started_at)
) STRICT;

-- Unique partial index: a user can have only one open (pending or active)
-- reading of a given edition at a time.
CREATE UNIQUE INDEX readings_one_open_per_user_edition_idx
    ON readings (user_id, edition_id)
    WHERE status IN ('pending', 'active');

