use serde::{Serialize, Deserialize};
use sqlx::{Row, FromRow};
use sqlx::sqlite::SqliteRow;

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
    #[serde(rename = "imageId")]
    pub image_url: String,
    #[serde(rename = "publicationDate")]
    pub publication_date: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "platform")]
    pub platform: String,
}

impl<'r> FromRow<'r, SqliteRow> for AppFavoriteWorldCard {
    fn from_row(row: &SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(AppFavoriteWorldCard {
            world_id: row.try_get("id")?,
            world_name: row.try_get("name")?,
            description: row.try_get("description")?,
            author_name: row.try_get("authorName")?,
            release_status: row.try_get("releaseStatus")?,
            recommended_capacity: row.try_get("recommendedCapacity")?,
            capacity: row.try_get("capacity")?,
            preview_youtube_id: row.try_get("previewYoutubeId")?,
            image_url: row.try_get("imageId")?,
            publication_date: row.try_get("publicationDate")?,
            updated_at: row.try_get("updated_at")?,
            platform: row.try_get("platform")?,
        })
    }
}
