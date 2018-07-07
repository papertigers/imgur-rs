use serde::{Deserialize, Deserializer};

/// Imgur Data Response
#[derive(Debug, Deserialize)]
pub struct Data<T> {
    /// Data field in imgur api response
    pub data: T,
}

/// Imgur Album
#[derive(Debug, Deserialize)]
pub struct Album {
    /// Album id
    pub id: String,
    /// Album title
    #[serde(deserialize_with = "null_string")]
    pub title: String,
    /// Album description
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    /// Album creation date
    pub datetime: u64,
    /// Vec of images in the album
    pub images: Vec<Image>,
}

/// Imgur Image
#[derive(Debug, Deserialize)]
pub struct Image {
    /// Image id
    pub id: String,
    /// Image title
    #[serde(deserialize_with = "null_string")]
    pub title: String,
    /// Image description
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    /// Image creation date
    pub datetime: u64,
    /// Image mime type
    #[serde(rename = "type")]
    pub mime: String,
    /// URL location of the image
    pub link: String,
}

/// Used to translate a `null` from the Imgur API as an empty string.
fn null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}
