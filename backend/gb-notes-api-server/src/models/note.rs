use serde::Deserialize;

#[derive(Deserialize)]
pub struct Note {
    #[serde(default)]
    id: u64,
    title: String,
    content: String,
}

impl Note {
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }
}
