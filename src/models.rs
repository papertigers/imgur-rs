use serde::{Deserialize, Deserializer};

/// A trait that signifies the type has an a url and filename
pub trait ProvidesFile {
    /// Get the download url from the type
    fn get_url(&self) -> &str;
    /// Get the filename from the type
    fn filename(&self) -> &str;
}

/// Imgur Data Response
///
/// Really just a container for an `Album` or an `Image`
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

/// Imgur Error
#[derive(Debug, Deserialize)]
pub struct ImgurError {
    /// Imgur's error message from the failed request
    pub error: String,
    /// Request path that failed
    pub request: String,
    /// HTTP Method of the failed request
    pub method: String,
}

/// Used to translate a `null` from the Imgur API as an empty string.
fn null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}

impl ProvidesFile for Image {
    /// Get the image's url
    fn get_url(&self) -> &str {
        self.link.as_str()
    }

    /// Get the image's filename, which is just the last part of the url path
    fn filename(&self) -> &str {
        self.get_url()
            .split("/")
            .last()
            .expect("imgur api provided bad data in the link field")
    }
}
