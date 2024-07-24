use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnityPackageFromAPI {
    #[serde(rename = "platform")]
    pub platform: String,
}