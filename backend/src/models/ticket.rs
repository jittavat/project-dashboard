use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "ticket_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TicketStatus {
    Backlog,
    Todo,
    InProgress,
    InReview,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type, PartialEq)]
#[sqlx(type_name = "ticket_priority", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum TicketPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Ticket {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub assignee_id: Option<Uuid>,
    pub reporter_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub finished_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub tags: Vec<String>,
    pub epic_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTicketRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
    pub assignee_id: Option<Uuid>,
    pub finished_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
    pub epic_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTicketRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub assignee_id: Option<Uuid>,
    pub finished_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
    pub epic_id: Option<Uuid>,
}
