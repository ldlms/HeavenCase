use argon2::password_hash::rand_core::{OsRng, RngCore};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{auth::hash_password, state::AppState};

type ApiError = (StatusCode, String);

fn internal<E: std::fmt::Display>(e: E) -> ApiError {
    tracing::error!("{e}");
    (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne".to_string())
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Compte créé, mail de validation envoyé", body = RegisterResponse),
        (status = 400, description = "Données invalides"),
        (status = 409, description = "Email déjà utilisé"),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), ApiError> {
    let username = req.username.trim().to_string();
    let email = req.email.trim().to_lowercase();

    if username.is_empty() || !email.contains('@') || req.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Pseudo requis, email valide et mot de passe de 8 caractères minimum".into(),
        ));
    }

    // Argon2 est volontairement lent : on le sort du thread async
    let password = req.password;
    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password))
        .await
        .map_err(internal)?
        .map_err(internal)?;

    // Utilisateur + jeton dans une transaction : tout passe ou rien
    let mut tx = state.pool.begin().await.map_err(internal)?;

    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            let duplicate = e.as_database_error().is_some_and(|d| d.is_unique_violation());
            if duplicate {
                (StatusCode::CONFLICT, "Cet email est déjà utilisé".to_string())
            } else {
                internal(e)
            }
        })?;

    // Jeton aléatoire envoyé par mail, seul son hash est stocké
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    let token = hex::encode(bytes);
    let token_hash = hex::encode(Sha256::digest(token.as_bytes()));

    sqlx::query(
        "INSERT INTO email_verification_tokens (token_hash, user_id, expires_at)
         VALUES ($1, $2, now() + interval '24 hours')",
    )
        .bind(&token_hash)
        .bind(user_id)
        .execute(&mut *tx)
        .await
        .map_err(internal)?;

    tx.commit().await.map_err(internal)?;

    let link = format!("{}/auth/verify?token={}", state.app_url, token);
    let body = format!(
        "Bienvenue {username} !\n\nClique sur ce lien pour valider ton compte (valable 24 h) :\n{link}"
    );
    if let Err(e) = state.mailer.send(&email, "Valide ton compte heavenCase", body).await {
        tracing::error!("Échec d'envoi du mail de validation : {e}");
    }

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: user_id,
            message: "Compte créé, vérifie tes mails".into(),
        }),
    ))
}