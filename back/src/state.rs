use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            app_name: "heavenCase".to_string(),
            pool,
        }
    }
}