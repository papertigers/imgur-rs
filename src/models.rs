use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    pub id: String,
    #[serde(deserialize_with = "null_string")]
    pub title: String,
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    pub datetime: u64,
    pub images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    pub id: String,
    #[serde(deserialize_with = "null_string")]
    pub title: String,
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    pub datetime: u64,
    #[serde(rename = "type")]
    pub mime: String,
    pub link: String,
}

fn null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}
