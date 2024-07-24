use serde::{Serialize, Deserialize};
use crate::structs::unity_package::UnityPackageFromAPI;

#[derive(Serialize, Deserialize, Debug)]
pub struct FavoriteWorldFromAPI {
    #[serde(rename = "id")]
    pub world_id: String,
    #[serde(rename = "name")]
    pub world_name: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "authorName")]
    pub author_name: String,
    #[serde(rename = "releaseStatus")]
    pub release_status: String,
    #[serde(rename = "recommendedCapacity")]
    pub recommended_capacity: Option<u32>,
    #[serde(rename = "capacity")]
    pub capacity: u32,
    #[serde(rename = "previewYoutubeId")]
    pub preview_youtube_id: Option<String>,
    #[serde(rename = "thumbnailImageUrl")]
    pub image_url: String,
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "unityPackages")]
    pub unity_packages: Option<Vec<UnityPackageFromAPI>>,
}