use serde::{Deserialize, Serialize};

#[tokio::test]
async fn correct_response() {
    let response = reqwest::get("http://localhost:3000").await.unwrap();
    let json_response = response.json::<Vec<PopularLanguage>>().await.unwrap();
    let languages_json = include_str!("../favorite_languages.json");
    let languages: Vec<PopularLanguage> = serde_json::from_str(languages_json).unwrap();
    assert_eq!(json_response.len(), 14);
    for (index, response_language) in json_response.iter().enumerate() {
        assert_eq!(response_language.name, languages[index].name);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PopularLanguage {
    pub name: String,
    pub popularity: f32,
}
