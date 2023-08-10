use std::collections::HashMap;

use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Result {
    Ok,
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ResponseType {
    Entity,
    Collection,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum EntityType {
    Manga,
    // more?
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PublicationDemographic {
    Shounen,
    Shoujo,
    Josei,
    Seinen,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MangaStatus {
    Reading,
    OnHold,
    PlanToRead,
    Dropped,
    Rereading,
    Completed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ContentRating {
    Safe,
    Suggestive,
    Erotica,
    Pornographic,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MangaState {
    Draft,
    Submitted,
    Published,
    Rejected,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MangaAttributes {
    title: HashMap<String, String>,
    alt_titles: Vec<HashMap<String, String>>,
    description: HashMap<String, String>,
    is_locked: bool,
    original_language: String,
    last_volume: String,
    last_chapter: String,
    publication_demographic: PublicationDemographic,
    status: MangaStatus,
    year: u32,
    content_rating: ContentRating,
    chapter_number_reset_on_new_volume: bool,
    available_translated_languages: Vec<String>,
    latest_uploaded_chapter: String,
    tags: Vec<HashMap<String, String>>,
    state: MangaState,
    version: u32, // to be safe
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct Manga {
    id: String,
    r#type: EntityType,
}

#[derive(Debug, Deserialize)]
struct MangaList {
    result: Result,
    response: ResponseType,
    data: Vec<Manga>,
    limit: u32,
    offset: u32,
    total: u32,
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.mangadex.org/manga")
        .query(&[("limit", 10)])
        .send()
        .await
        .unwrap()
        .json::<MangaList>()
        .await
        .unwrap();

    println!("{:#?}", response)
}
