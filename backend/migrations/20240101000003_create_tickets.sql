CREATE TYPE ticket_status AS ENUM (
    'backlog', 'todo', 'in_progress', 'in_review', 'done'
);

CREATE TYPE ticket_priority AS ENUM (
    'low', 'medium', 'high', 'critical'
);

CREATE TABLE IF NOT EXISTS tickets (
    id          UUID            PRIMARY KEY DEFAULT uuid_generate_v4(),
    project_id  UUID            NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title       TEXT            NOT NULL,
    description TEXT,
    status      ticket_status   NOT NULL DEFAULT 'backlog',
    priority    ticket_priority NOT NULL DEFAULT 'medium',
    assignee_id UUID            REFERENCES users(id) ON DELETE SET NULL,
    reporter_id UUID            NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_tickets_project  ON tickets(project_id);
CREATE INDEX idx_tickets_assignee ON tickets(assignee_id);
CREATE INDEX idx_tickets_status   ON tickets(project_id, status);
