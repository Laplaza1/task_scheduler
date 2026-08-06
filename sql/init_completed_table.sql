CREATE TABLE IF NOT EXISTS completed_tasks (
            id              SERIAL PRIMARY KEY,
            task_id         INTEGER NOT NULL,
            date            DATE,
            completetor     TEXT NOT NULL,
            created_at      DATE NOT NULL DEFAULT NOW(),
            updated_at      DATE NOT NULL DEFAULT NOW());