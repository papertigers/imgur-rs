use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct Data<T> {
    data: T,
}

#[derive(Debug, Deserialize)]
pub struct Album {
    id: String,
    #[serde(deserialize_with = "null_string")]
    title: String,
    #[serde(deserialize_with = "null_string")]
    description: String,
    datetime: u64,
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
pub struct Image {
    id: String,
    #[serde(deserialize_with = "null_string")]
    title: String,
    #[serde(deserialize_with = "null_string")]
    description: String,
    datetime: u64,
    #[serde(rename = "type")]
    mime: String,
    link: String,
}

fn null_string<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| x.unwrap_or("".to_string()))
}
