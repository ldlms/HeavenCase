mod errors;
mod mailer;
mod routes;
mod state;

use state::AppState;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};
use tower_sessions_sqlx_store::PostgresStore;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool =
        sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL manquante"))
            .await
            .expect("Connexion à la base impossible");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Échec des migrations");

    let session_store = PostgresStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("Échec de la création de la table des sessions");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false) // passer à true en prod (HTTPS)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    let state = AppState::new(pool, mailer::Mailer::from_env());

    let app = routes::router()
        .with_state(state)
        .layer(session_layer)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    tracing::info!("Serveur sur http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}
