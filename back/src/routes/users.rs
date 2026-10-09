use axum::{extract::{Path, State}, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{errors::AppError, state::AppState};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    nom: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one))
}

async fn list() -> Json<Vec<User>> {
    Json(vec![User { id: 1, nom: "Leo".into() }])
}

async fn get_one(Path(id): Path<u32>) -> Result<Json<User>, AppError> {
    if id == 1 {
        Ok(Json(User { id, nom: "Leo".into() }))
    } else {
        Err(AppError::NotFound(format!("User {id} introuvable")))
    }
}

async fn create(State(_state): State<AppState>, Json(user): Json<User>) -> Json<User> {
    Json(user) // ici : insertion en base via state.db
}