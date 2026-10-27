use sqlx::PgPool;
use crate::mailer::Mailer;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub pool: PgPool,
    pub mailer: Mailer,
    pub app_url: String,
}

impl AppState {
    pub fn new(pool: PgPool, mailer: Mailer) -> Self {
        Self {
            app_name: "heavenCase".to_string(),
            pool,
            mailer,
            app_url: std::env::var("APP_URL").expect("APP_URL manquante"),
        }
    }
}