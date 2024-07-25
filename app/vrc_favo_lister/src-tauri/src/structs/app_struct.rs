use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AppFavoriteWorldCard {
    #[serde(rename = "id")]
    pub world_id: String,
    #[serde(rename = "name")]
    pub world_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "releaseStatus")]
    pub release_status: String,
    #[serde(rename = "recommendedCapacity")]
    pub recommended_capacity: u32,
    #[serde(rename = "capacity")]
    pub capacity: u32,
    #[serde(rename = "previewYoutubeId")]
    pub preview_youtube_id: Option<String>,
    #[serde(rename = "thumbnailImageUrl")]
    pub image_url: String,
    #[serde(rename = "publication_date")]
    pub publication_date: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "platform")]
    pub platform: String,
}