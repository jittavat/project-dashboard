CREATE TABLE epics (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id    UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title         TEXT NOT NULL,
    description   TEXT,
    status        ticket_status   NOT NULL DEFAULT 'backlog',
    priority      ticket_priority NOT NULL DEFAULT 'medium',
    assignee_id   UUID REFERENCES users(id) ON DELETE SET NULL,
    reporter_id   UUID NOT NULL REFERENCES users(id),
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    finished_date TIMESTAMPTZ,
    start_date    TIMESTAMPTZ,
    assignee      TEXT,
    team          TEXT,
    tags          TEXT[] NOT NULL DEFAULT '{}'
);

ALTER TABLE tickets ADD COLUMN epic_id UUID REFERENCES epics(id) ON DELETE SET NULL;

CREATE INDEX idx_epics_project ON epics(project_id);
CREATE INDEX idx_tickets_epic  ON tickets(epic_id);
