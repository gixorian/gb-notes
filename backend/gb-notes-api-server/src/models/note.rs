use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;

use crate::models::database::Database;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Note {
    #[serde(default)]
    id: u64,
    title: String,
    content: String,
    #[serde(default)]
    created_at: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    success: bool,
    id: u64,
}

pub async fn get_notes(State(db): State<Arc<Database>>) -> Json<Vec<Note>> {
    let notes = sqlx::query_as::<_, Note>("SELECT id, title, content, created_at FROM notes")
        .fetch_all(&db.pool)
        .await
        .unwrap();
    axum::Json(notes)
}

impl Note {
    pub async fn create_note_handler(
        State(db): State<Arc<Database>>,
        Json(new_note): Json<Note>,
    ) -> (StatusCode, Json<ApiResponse>) {
        let id = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO notes (title, content) VALUES (?, ?) RETURNING id",
        )
        .bind(&new_note.title)
        .bind(&new_note.content)
        .fetch_one(&db.pool)
        .await
        .unwrap()
        .0;

        (
            StatusCode::CREATED,
            Json(ApiResponse {
                success: true,
                id: id.try_into().unwrap(),
            }),
        )
    }
    pub fn _set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn _get_id(&self) -> u64 {
        self.id
    }

    pub fn _get_title(&self) -> &str {
        &self.title
    }

    pub fn _get_content(&self) -> &str {
        &self.content
    }
}
