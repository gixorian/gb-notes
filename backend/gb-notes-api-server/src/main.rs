use axum::{Router, routing::get};

use crate::handlers::new_note_handler::create_note_handler;
mod models {
    pub mod note;
}
mod handlers {
    pub mod new_note_handler;
}

#[tokio::main]
async fn main() {
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
            get(|| async { "New note endpoint" }).post(create_note_handler),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
