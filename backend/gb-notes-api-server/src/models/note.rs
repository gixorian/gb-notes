use serde::Deserialize;

#[derive(Deserialize)]
pub struct Note {
    #[serde(default)]
    pub id: u64,
    pub title: String,
    pub content: String,
}
