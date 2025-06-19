use sqlx::sqlite::SqlitePoolOptions;

pub struct Database {
    pub pool: sqlx::SqlitePool,
}

impl Database {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let opts = SqlitePoolOptions::new()
            .connect(&format!("sqlite:{}", db_path))
            .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL
            );",
        )
        .execute(&opts)
        .await?;

        Ok(Database { pool: opts })
    }
}
