CREATE TABLE IF NOT EXISTS users (
    id              SERIAL PRIMARY KEY,
    _name            TEXT NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    created_at      DATE NOT NULL DEFAULT NOW(),
    updated_at      DATE NOT NULL DEFAULT NOW()
);
        