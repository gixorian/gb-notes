use axum::{Router, extract::Json, http::StatusCode, routing::post};
use serde::Serialize;

use crate::models::note::Note;

#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
    id: u64,
}

pub async fn create_note_handler(Json(payload): Json<Note>) -> (StatusCode, Json<ApiResponse>) {
    let Note {
        id: _,
        title,
        content,
    } = payload;
    println!("Received note: Title: {}, Content: {}", title, content);

    // TODO: Get an incremental ID from the database
    let new_id = 1;

    (
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            id: new_id,
        }),
    )
}
