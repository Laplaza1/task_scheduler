CREATE TABLE IF NOT EXISTS tasks (
            id              SERIAL PRIMARY KEY,
            user_id        INTEGER NOT NULL,
            task            TEXT NOT NULL,
            description      TEXT,
            due_date        DATE,
            created_at      DATE NOT NULL DEFAULT NOW(),
            updated_at      DATE NOT NULL DEFAULT NOW(),
            weight          INTEGER DEFAULT 100,
            CONSTRAINT valid_due_date
                CHECK (due_date IS NULL OR due_date > created_at),

            CONSTRAINT valid_user_id
                FOREIGN KEY (user_id)
                REFERENCES users(id)
                );