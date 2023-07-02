use axum::Json;
use serde::{Deserialize, Serialize};

const LANGUAGE_LIST: &str = include_str!("../../favorite_languages.json");

#[derive(Serialize, Deserialize)]
pub struct FavoriteLanguage {
    pub name: String,
    pub popularity: f32,
}

pub async fn favorite_languages() -> Json<Vec<FavoriteLanguage>> {
    let languages: Vec<FavoriteLanguage> = serde_json::from_str(LANGUAGE_LIST).unwrap();

    Json(languages)
}
