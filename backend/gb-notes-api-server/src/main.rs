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
        .route("/", get(get_root))
        .route("/health", get(get_health))
        .route("/help", get(get_help))
        .route("/notes", get(get_notes))
        .route("/new-note", get(get_new_note).post(create_note_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn post_new_note() -> &'static str {
    "New note created"
}

async fn get_root() -> &'static str {
    "Welcome to the GB Notes API Server!"
}

async fn get_health() -> &'static str {
    "Health check: OK"
}

async fn get_help() -> &'static str {
    "API Help: Use /api/notes to get notes and /api/new-note to create a new note."
}

async fn get_notes() -> &'static str {
    "Notes endpoint"
}

async fn get_new_note() -> &'static str {
    "New note endpoint"
}
