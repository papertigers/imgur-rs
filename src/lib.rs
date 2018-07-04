#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate failure;
extern crate reqwest;

mod models;
use models::{Album, Data};

use std::fmt;

use failure::Error;
use reqwest::{header, Client};

const API_BASE: &'static str = "https://api.imgur.com/3/";

/// macro to easily generate the full path for the http client
macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", API_BASE, $path).as_str()
        );
    );

/// Handle to the imgur api
pub struct ImgurHandle {
    client_id: String,
    client: Client,
}

impl ImgurHandle {
    /// Create a new Imgur handle
    ///
    /// * `client_id` - Client ID required by the Imgur API.
    pub fn new(client_id: String) -> Self {
        let mut headers = header::Headers::new();
        headers.set(header::Authorization(
            format!("Client-ID {}", client_id).to_string(),
        ));
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build client");

        ImgurHandle {
            client_id: client_id,
            client: client,
        }
    }

    pub fn get_album(&self, id: &str) -> Result<Data<Album>, Error> {
        let album: Data<Album> = self.client
            .get(api_url!(format!("{}/{}", "album", id)))
            .send()?
            .json()?;

        Ok(album)
    }
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}
