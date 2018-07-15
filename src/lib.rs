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
use models::ImgurError;
pub use models::{Album, Data, Image, ProvidesFile};

use std::fmt;
use std::io;
use std::io::Write;

use failure::Error;
use reqwest::header::Headers;
use reqwest::{header, Client, StatusCode};
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
    api_client: Client,
    download_client: Client,
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

        // API client sets an Authorization header
        let api_client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build api client");

        // Used exclusively for downloading content
        let download_client = Client::builder()
            .build()
            .expect("failed to build download client");

        ImgurHandle {
            client_id: client_id,
            api_client: api_client,
            download_client: download_client,
        }
    }

    fn api_request<T>(&self, path: &str) -> Result<Data<T>, Error>
    where
        for<'de> T: Deserialize<'de>,
    {
        let mut res = self.api_client.get(api_url!(path)).send()?;
        if res.status().is_success() {
            let data: Data<T> = res.json()?;
            return Ok(data);
        }

        let ires: Data<ImgurError> = res.json()?;
        Err(imgur_error_to_failure(ires.data, res.status()))
    }

    fn download_request<U, W: ?Sized>(&self, item: &U, w: &mut W) -> Result<u64, Error>
    where
        U: ProvidesFile,
        W: Write,
    {
        let mut res = self.download_client
            .get(item.get_url())
            .headers(Headers::new()) // Clear Client-ID
            .send()?;

        if !res.status().is_success() {
            let ires: Data<ImgurError> = res.json()?;
            return Err(imgur_error_to_failure(ires.data, res.status()));
        }

        match io::copy(&mut res, w) {
            Ok(b) => Ok(b),
            // For now we rely on `Display`, if in the future we need a better way to deal with
            // this we can implement a custom error type
            Err(e) => Err(format_err!("{}", e)),
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
    pub fn download_image<W: ?Sized>(&self, i: &Image, w: &mut W) -> Result<u64, Error>
    where
        W: Write,
    {
        self.download_request(i, w)
    }
}

fn imgur_error_to_failure(e: ImgurError, s: StatusCode) -> Error {
    format_err!("{}: {} ({})", s, e.error, e.request)
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}
