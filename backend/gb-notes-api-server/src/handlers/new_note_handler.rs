use axum::{extract::Json, http::StatusCode};
use serde::Serialize;

use crate::models::note::Note;

#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
    id: u64,
}

pub async fn create_note_handler(
    Json(mut new_note): Json<Note>,
) -> (StatusCode, Json<ApiResponse>) {
    new_note.set_id(1); // Set the ID to a new incremental value
    println!(
        "Received note: Title: {}, Content: {}",
        new_note.get_title(),
        new_note.get_content()
    );

    // TODO: Get an incremental ID from the database

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            id: new_note.get_id(),
        }),
    )
}
