mod auth;
mod users;

use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(db_health, auth::register, auth::verify_email, auth::login),
    tags(
        (name = "health", description = "État du service"),
        (name = "auth", description = "Inscription et connexion")
    )
)]
pub struct ApiDoc;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/health/db", get(db_health))
        .nest("/users", users::router())
        .route("/auth/register", post(auth::register))
        .route("/auth/verify", get(auth::verify_email))
        .route("/auth/login", post(auth::login))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}

#[utoipa::path(
    get,
    path = "/health/db",
    tag = "health",
    responses((status = 200, description = "Tables présentes en base", body = Vec<String>))
)]
async fn db_health(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    let tables: Vec<String> = sqlx::query_scalar(
        "SELECT table_name::text FROM information_schema.tables
         WHERE table_schema = 'public' ORDER BY table_name",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tables))
}
