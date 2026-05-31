use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{auth, epics, projects, tickets};
use crate::models::{epic::*, project::*, ticket::*, user::*};

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        projects::list_projects,
        projects::create_project,
        projects::update_project,
        tickets::list_tickets,
        tickets::create_ticket,
        tickets::get_ticket,
        tickets::update_ticket,
        tickets::delete_ticket,
        epics::list_epics,
        epics::create_epic,
        epics::get_epic,
        epics::update_epic,
        epics::delete_epic,
    ),
    components(schemas(
        RegisterRequest,
        LoginRequest,
        AuthResponse,
        UserPublic,
        Project,
        CreateProjectRequest,
        UpdateProjectRequest,
        Ticket,
        CreateTicketRequest,
        UpdateTicketRequest,
        TicketStatus,
        TicketPriority,
        Epic,
        CreateEpicRequest,
        UpdateEpicRequest,
    )),
    tags(
        (name = "auth",     description = "Authentication"),
        (name = "projects", description = "Project management"),
        (name = "tickets",  description = "Ticket management"),
        (name = "epics",    description = "Epic management"),
    )
)]
pub struct ApiDoc;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi));

    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login)),
            )
            .service(
                web::scope("/projects")
                    .route("", web::get().to(projects::list_projects))
                    .route("", web::post().to(projects::create_project))
                    .route("/{project_id}", web::put().to(projects::update_project))
                    .service(
                        web::scope("/{project_id}/tickets")
                            .route("", web::get().to(tickets::list_tickets))
                            .route("", web::post().to(tickets::create_ticket))
                            .route("/{ticket_id}", web::get().to(tickets::get_ticket))
                            .route("/{ticket_id}", web::put().to(tickets::update_ticket))
                            .route("/{ticket_id}", web::delete().to(tickets::delete_ticket)),
                    )
                    .service(
                        web::scope("/{project_id}/epics")
                            .route("", web::get().to(epics::list_epics))
                            .route("", web::post().to(epics::create_epic))
                            .route("/{epic_id}", web::get().to(epics::get_epic))
                            .route("/{epic_id}", web::put().to(epics::update_epic))
                            .route("/{epic_id}", web::delete().to(epics::delete_epic)),
                    ),
            ),
    );
}
