#![allow(dependency_on_unit_never_type_fallback)]

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    db::DbPool,
    errors::AppError,
    middleware::auth::CurrentUser,
    models::project::{CreateProjectRequest, Project, UpdateProjectRequest},
};

/// List all projects for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/projects",
    responses(
        (status = 200, description = "List of projects", body = Vec<Project>),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
pub async fn list_projects(
    pool: web::Data<DbPool>,
    user: CurrentUser,
) -> Result<HttpResponse, AppError> {
    let projects = sqlx::query_as!(
        Project,
        "SELECT * FROM projects WHERE owner_id = $1 ORDER BY created_at DESC",
        user.0.sub
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(projects))
}

/// Create a new project
#[utoipa::path(
    post,
    path = "/api/v1/projects",
    request_body = CreateProjectRequest,
    responses(
        (status = 201, description = "Project created", body = Project),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
pub async fn create_project(
    pool: web::Data<DbPool>,
    user: CurrentUser,
    body: web::Json<CreateProjectRequest>,
) -> Result<HttpResponse, AppError> {
    if body.name.trim().is_empty() {
        return Err(AppError::Validation("Project name cannot be empty".into()));
    }
    let key = body.key.to_uppercase();

    let project = sqlx::query_as!(
        Project,
        r#"INSERT INTO projects (id, name, key, description, owner_id, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
           RETURNING *"#,
        Uuid::new_v4(),
        body.name.trim(),
        key,
        body.description,
        user.0.sub,
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db) if db.constraint() == Some("projects_key_key") => {
            AppError::Validation(format!("Project key '{}' already exists", key))
        }
        _ => AppError::Database(e),
    })?;

    Ok(HttpResponse::Created().json(project))
}

/// Update a project (name and/or description)
#[utoipa::path(
    put,
    path = "/api/v1/projects/{project_id}",
    params(("project_id" = Uuid, Path, description = "Project ID")),
    request_body = UpdateProjectRequest,
    responses(
        (status = 200, description = "Project updated", body = Project),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Project not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "projects"
)]
pub async fn update_project(
    pool: web::Data<DbPool>,
    user: CurrentUser,
    project_id: web::Path<Uuid>,
    body: web::Json<UpdateProjectRequest>,
) -> Result<HttpResponse, AppError> {
    let project = sqlx::query_as!(
        Project,
        r#"UPDATE projects
           SET name        = COALESCE($1, name),
               description = COALESCE($2, description),
               updated_at  = NOW()
           WHERE id = $3 AND owner_id = $4
           RETURNING *"#,
        body.name.as_deref(),
        body.description.as_deref(),
        *project_id,
        user.0.sub,
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| AppError::NotFound("Project not found".into()))?;

    Ok(HttpResponse::Ok().json(project))
}
