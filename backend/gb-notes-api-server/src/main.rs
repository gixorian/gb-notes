use std::sync::Arc;

use crate::models::note::Note;
use axum::{Router, routing::get};

mod models {
    pub mod database;
    pub mod note;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:///home/goran/projects/fullstack-dev/gb-notes/backend/notes.db"; // Use the path variable if needed
    let db = Arc::new(models::database::Database::new(database_url).await?);

    let app = Router::new()
        .route("/", get(|| async { "Welcome to the GB Notes API Server!" }))
        .route("/health", get(|| async { "Health check: OK" }))
        .route(
            "/help",
            get(|| async {
                "API Help: Use /api/notes to get notes and /api/new-note to create a new note."
            }),
        )
        .route("/notes", get(|| async { "Notes endpoint" }))
        .route(
            "/new-note",
            get(|| async { "New note endpoint" }).post(Note::create_note_handler),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
