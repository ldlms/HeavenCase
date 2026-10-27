mod routes;
mod state;
mod errors;

use state::AppState;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL manquante"))
        .await
        .expect("Connexion à la base impossible");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Échec des migrations");

    let state = AppState::new(pool);

    let app = routes::router()
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    tracing::info!("Serveur sur http://localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}