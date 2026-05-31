use sqlx::postgres::PgPoolOptions;

pub type DbPool = sqlx::PgPool;

pub async fn connect(database_url: &str) -> anyhow::Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(20)
        // min_connections(0) = lazy connect; the pool only opens connections on
        // first use, so startup never races against postgres initialization.
        .min_connections(0)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await?;
    Ok(pool)
}
