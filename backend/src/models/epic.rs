use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::models::ticket::{TicketPriority, TicketStatus};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::FromRow)]
pub struct Epic {
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
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEpicRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: TicketPriority,
    pub assignee_id: Option<Uuid>,
    pub finished_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEpicRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TicketStatus>,
    pub priority: Option<TicketPriority>,
    pub assignee_id: Option<Uuid>,
    pub finished_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
}
