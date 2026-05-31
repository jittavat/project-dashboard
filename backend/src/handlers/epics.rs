#![allow(dependency_on_unit_never_type_fallback)]

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::AppError,
    middleware::auth::CurrentUser,
    models::epic::{CreateEpicRequest, Epic, UpdateEpicRequest},
};

/// List all epics for a project
#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}/epics",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    responses(
        (status = 200, description = "List of epics", body = Vec<Epic>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "epics"
)]
pub async fn list_epics(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let project_id = path.into_inner();
    let epics = sqlx::query_as!(
        Epic,
        r#"SELECT id, project_id, title, description,
                  status AS "status: _", priority AS "priority: _",
                  assignee_id, reporter_id, created_at, updated_at,
                  finished_date, start_date, assignee, tags
           FROM epics
           WHERE project_id = $1
           ORDER BY created_at DESC"#,
        project_id
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(epics))
}

/// Create a new epic in a project
#[utoipa::path(
    post,
    path = "/api/v1/projects/{project_id}/epics",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    request_body = CreateEpicRequest,
    responses(
        (status = 201, description = "Epic created", body = Epic),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "epics"
)]
pub async fn create_epic(
    pool: web::Data<DbPool>,
    user: CurrentUser,
    path: web::Path<Uuid>,
    body: web::Json<CreateEpicRequest>,
) -> Result<HttpResponse, AppError> {
    let project_id = path.into_inner();
    if body.title.trim().is_empty() {
        return Err(AppError::Validation("Epic title cannot be empty".into()));
    }

    let epic = sqlx::query_as!(
        Epic,
        r#"INSERT INTO epics
               (id, project_id, title, description, priority, assignee_id, reporter_id,
                finished_date, start_date, assignee, tags,
                created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, COALESCE($11, '{}'::text[]), NOW(), NOW())
           RETURNING id, project_id, title, description,
                     status AS "status: _", priority AS "priority: _",
                     assignee_id, reporter_id, created_at, updated_at,
                     finished_date, start_date, assignee, tags"#,
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
    )
    .fetch_one(pool.get_ref())
    .await?;

    Ok(HttpResponse::Created().json(epic))
}

/// Get a single epic
#[utoipa::path(
    get,
    path = "/api/v1/projects/{project_id}/epics/{epic_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("epic_id"    = Uuid, Path, description = "Epic ID"),
    ),
    responses(
        (status = 200, description = "Epic", body = Epic),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "epics"
)]
pub async fn get_epic(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (project_id, epic_id) = path.into_inner();
    let epic = sqlx::query_as!(
        Epic,
        r#"SELECT id, project_id, title, description,
                  status AS "status: _", priority AS "priority: _",
                  assignee_id, reporter_id, created_at, updated_at,
                  finished_date, start_date, assignee, tags
           FROM epics
           WHERE id = $1 AND project_id = $2"#,
        epic_id,
        project_id,
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Epic {} not found", epic_id)))?;

    Ok(HttpResponse::Ok().json(epic))
}

/// Update an epic
#[utoipa::path(
    put,
    path = "/api/v1/projects/{project_id}/epics/{epic_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("epic_id"    = Uuid, Path, description = "Epic ID"),
    ),
    request_body = UpdateEpicRequest,
    responses(
        (status = 200, description = "Updated epic", body = Epic),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "epics"
)]
pub async fn update_epic(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
    body: web::Json<UpdateEpicRequest>,
) -> Result<HttpResponse, AppError> {
    let (project_id, epic_id) = path.into_inner();

    let epic = sqlx::query_as!(
        Epic,
        r#"UPDATE epics
           SET title         = COALESCE($3, title),
               description   = COALESCE($4, description),
               status        = COALESCE($5, status),
               priority      = COALESCE($6, priority),
               assignee_id   = COALESCE($7, assignee_id),
               finished_date = COALESCE($8, finished_date),
               start_date    = COALESCE($9, start_date),
               assignee      = COALESCE($10, assignee),
               tags          = COALESCE($11, tags),
               updated_at    = NOW()
           WHERE id = $1 AND project_id = $2
           RETURNING id, project_id, title, description,
                     status AS "status: _", priority AS "priority: _",
                     assignee_id, reporter_id, created_at, updated_at,
                     finished_date, start_date, assignee, tags"#,
        epic_id,
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
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Epic {} not found", epic_id)))?;

    Ok(HttpResponse::Ok().json(epic))
}

/// Delete an epic
#[utoipa::path(
    delete,
    path = "/api/v1/projects/{project_id}/epics/{epic_id}",
    params(
        ("project_id" = Uuid, Path, description = "Project ID"),
        ("epic_id"    = Uuid, Path, description = "Epic ID"),
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 404, description = "Not found"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "epics"
)]
pub async fn delete_epic(
    pool: web::Data<DbPool>,
    _user: CurrentUser,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (project_id, epic_id) = path.into_inner();
    let rows = sqlx::query!(
        "DELETE FROM epics WHERE id = $1 AND project_id = $2",
        epic_id,
        project_id
    )
    .execute(pool.get_ref())
    .await?
    .rows_affected();

    if rows == 0 {
        return Err(AppError::NotFound(format!("Epic {} not found", epic_id)));
    }
    Ok(HttpResponse::NoContent().finish())
}
