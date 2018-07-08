#![deny(warnings)]
#![deny(missing_docs)]

//! # imgur_rs
//!
//! The `imgur_rs` crate provides a high-level handle to the Imgur API by leveraging the `reqwest`
//! crate for making HTTP requests.

#[macro_use]
extern crate serde_derive;
extern crate serde;

#[macro_use]
extern crate failure;

extern crate reqwest;

mod models;
pub use models::{Album, Data, Image, ProvidesFile};

use std::fmt;
use std::io;
use std::io::Write;

use failure::Error;
use reqwest::header::Headers;
use reqwest::{header, Client};
use serde::Deserialize;

const API_BASE: &'static str = "https://api.imgur.com/3/";

/// macro to easily generate the full path for the http client
macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", API_BASE, $path).as_str()
        );
    );

/// Handle to the Imgur API that allows you to get albums and images.
///
/// # Example
/// ```
/// let handle = imgur_rs::ImgurHandle::new("Client-ID".to_string());
/// ```
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

    fn api_request<T>(&self, path: &str) -> Result<Data<T>, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut res = self.client.get(api_url!(path)).send()?;

        // TODO handle status code before json parse

        let data: Data<T> = res.json()?;
        Ok(data)
    }

    fn download_request<U, W: ?Sized>(&self, item: &U, w: &mut W) -> Result<u64, Error>
    where
        U: ProvidesFile,
        W: Write,
    {
        let mut res = self.client
            .get(item.get_url())
            .headers(Headers::new()) // Clear Client-ID
            .send()?;
        match io::copy(&mut res, w) {
            Ok(b) => Ok(b),
            Err(_) => Err(format_err!("error writing to destination")),
        }
    }

    /// Get an imgur image by id
    pub fn get_image(&self, id: &str) -> Result<Data<Image>, Error> {
        self.api_request(format!("image/{}", id).as_str())
    }

    /// Get an imgur album by id
    pub fn get_album(&self, id: &str) -> Result<Data<Album>, Error> {
        self.api_request(format!("album/{}", id).as_str())
    }

    /// Get an imgur gallery by id which is really just an alias for an imgur album
    pub fn get_gallery_as_album(&self, id: &str) -> Result<Data<Album>, Error> {
        self.get_album(id)
    }

    /// Download an Imgur image
    pub fn download_image<W: ?Sized>(&self, i: &Image, p: &mut W) -> Result<u64, Error>
    where
        W: Write,
    {
        self.download_request(i, p)
    }
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}
