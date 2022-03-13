use serde::Deserialize;

#[derive(Deserialize)]
pub struct Note {
    pub message: String,
}
