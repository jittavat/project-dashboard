#![allow(dependency_on_unit_never_type_fallback)]

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::AppError,
    middleware::auth::CurrentUser,
    models::ticket::{CreateTicketRequest, Ticket, UpdateTicketRequest},
};

/// List all tickets for a project
#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}/tickets",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    responses(
        (status = 200, description = "List of tickets", body = Vec<Ticket>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "tickets"
)]
pub async fn list_tickets(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let project_id = path.into_inner();
    let tickets = sqlx::query_as!(
        Ticket,
        r#"SELECT id, project_id, title, description,
                  status AS "status: _", priority AS "priority: _",
                  assignee_id, reporter_id, created_at, updated_at,
                  finished_date, start_date, assignee, tags, epic_id
           FROM tickets
           WHERE project_id = $1
           ORDER BY created_at DESC"#,
        project_id
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(tickets))
}

/// Create a new ticket in a project
#[utoipa::path(
    post,
    path = "/api/v1/projects/{project_id}/tickets",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    request_body = CreateTicketRequest,
    responses(
        (status = 201, description = "Ticket created", body = Ticket),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "tickets"
)]
pub async fn create_ticket(
    pool: web::Data<DbPool>,
    user: CurrentUser,
    path: web::Path<Uuid>,
    body: web::Json<CreateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let project_id = path.into_inner();
    if body.title.trim().is_empty() {
        return Err(AppError::Validation("Ticket title cannot be empty".into()));
    }

    let ticket = sqlx::query_as!(
        Ticket,
        r#"INSERT INTO tickets
               (id, project_id, title, description, priority, assignee_id, reporter_id,
                finished_date, start_date, assignee, tags, epic_id,
                created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, COALESCE($11, '{}'::text[]), $12, NOW(), NOW())
           RETURNING id, project_id, title, description,
                     status AS "status: _", priority AS "priority: _",
                     assignee_id, reporter_id, created_at, updated_at,
                     finished_date, start_date, assignee, tags, epic_id"#,
        Uuid::new_v4(),
        project_id,
        body.title.trim(),
        body.description,
        body.priority as _,
        body.assignee_id,
        user.0.sub,
        body.finished_date,
        body.start_date,
        body.assignee,
        body.tags.as_deref(),
        body.epic_id,
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(ticket))
}

/// Get a single ticket
#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}/tickets/{ticket_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("ticket_id"  = Uuid, Path, description = "Ticket ID"),
    ),
    responses(
        (status = 200, description = "Ticket", body = Ticket),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "tickets"
)]
pub async fn get_ticket(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (project_id, ticket_id) = path.into_inner();
    let ticket = sqlx::query_as!(
        Ticket,
        r#"SELECT id, project_id, title, description,
                  status AS "status: _", priority AS "priority: _",
                  assignee_id, reporter_id, created_at, updated_at,
                  finished_date, start_date, assignee, tags, epic_id
           FROM tickets
           WHERE id = $1 AND project_id = $2"#,
        ticket_id,
        project_id,
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", ticket_id)))?;

    Ok(HttpResponse::Ok().json(ticket))
}

/// Update a ticket
#[utoipa::path(
    put,
    path = "/api/v1/projects/{project_id}/tickets/{ticket_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("ticket_id"  = Uuid, Path, description = "Ticket ID"),
    ),
    request_body = UpdateTicketRequest,
    responses(
        (status = 200, description = "Updated ticket", body = Ticket),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "tickets"
)]
pub async fn update_ticket(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateTicketRequest>,
) -> Result<HttpResponse, AppError> {
    let (project_id, ticket_id) = path.into_inner();

    let ticket = sqlx::query_as!(
        Ticket,
        r#"UPDATE tickets
           SET title         = COALESCE($3, title),
               description   = COALESCE($4, description),
               status        = COALESCE($5, status),
               priority      = COALESCE($6, priority),
               assignee_id   = COALESCE($7, assignee_id),
               finished_date = COALESCE($8, finished_date),
               start_date    = COALESCE($9, start_date),
               assignee      = COALESCE($10, assignee),
               tags          = COALESCE($11, tags),
               epic_id       = COALESCE($12, epic_id),
               updated_at    = NOW()
           WHERE id = $1 AND project_id = $2
           RETURNING id, project_id, title, description,
                     status AS "status: _", priority AS "priority: _",
                     assignee_id, reporter_id, created_at, updated_at,
                     finished_date, start_date, assignee, tags, epic_id"#,
        ticket_id,
        project_id,
        body.title,
        body.description,
        body.status as _,
        body.priority as _,
        body.assignee_id,
        body.finished_date,
        body.start_date,
        body.assignee,
        body.tags.as_deref(),
        body.epic_id,
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Ticket {} not found", ticket_id)))?;

    Ok(HttpResponse::Ok().json(ticket))
}

/// Delete a ticket
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{project_id}/tickets/{ticket_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("ticket_id"  = Uuid, Path, description = "Ticket ID"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "tickets"
)]
pub async fn delete_ticket(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (project_id, ticket_id) = path.into_inner();
    let rows = sqlx::query!(
        "DELETE FROM tickets WHERE id = $1 AND project_id = $2",
        ticket_id,
        project_id
    )
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Ticket {} not found", ticket_id)));
    }
    Ok(HttpResponse::NoContent().finish())
}
