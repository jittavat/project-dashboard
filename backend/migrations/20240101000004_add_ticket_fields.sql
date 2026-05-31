ALTER TABLE tickets
    ADD COLUMN finished_date TIMESTAMPTZ,
    ADD COLUMN assignee      TEXT,
    ADD COLUMN team          TEXT,
    ADD COLUMN tags          TEXT[] NOT NULL DEFAULT '{}';
