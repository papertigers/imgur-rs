#[macro_use]
extern crate serde_derive;
extern crate serde;

extern crate failure;
extern crate reqwest;

mod models;
use models::{Album, Data, Image};

use std::fmt;

use failure::Error;
use serde::Deserialize;
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

    fn raw_request<T>(&self, path: &str) -> Result<Data<T>, Error>
        where for<'de> T: Deserialize<'de>
    {
        let res: Data<T> = self.client
            .get(api_url!(path))
            .send()?
            .json()?;

        // TODO handle status code before json parse

        Ok(res)
    }

    /// Get an imgur image by id
    pub fn get_image(&self, id: &str) -> Result<Data<Image>, Error> {
        self.raw_request(api_url!(format!("image/{}", id)))
    }

    /// Get an imgur album by id
    pub fn get_album(&self, id: &str) -> Result<Data<Album>, Error> {
        self.raw_request(api_url!(format!("album/{}", id)))
    }

    /// Get an imgur gallery by id which is really just an alias for an imgur album
    pub fn get_gallery(&self, id: &str) -> Result<Data<Album>, Error> {
        self.get_album(id)
    }
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}
