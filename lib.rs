extern crate hyper;
extern crate serde_json as json;

use std::fmt;
use hyper::Client;
use hyper::header::{Headers, Authorization};
use json::Value;


const API_BASE: &'static str = "https://api.imgur.com/3/";
macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", API_BASE, $path)
        );
    );

/// Handle to the imgur api
pub struct ImgurHandle {
    client_id: String,
    client: Client,
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}

impl ImgurHandle {
    /// Create a new Imgur handle
    ///
    /// * `client_id` - Client ID required by the Imgur API.
    pub fn new(client_id: String) -> Self {
        let client = Client::new();
        ImgurHandle {
            client_id: client_id,
            client: client
        }
    }

    /// Retrieves imgur image urls from an album based on album id
    ///
    /// * `album_id` - Imgur album id.
    pub fn get_album(&self, album_id: String) -> Result<Vec<String>, ImgurError> {
        let mut headers = Headers::new();
        headers.set(Authorization(format!("Client-ID {}", self.client_id.to_owned())));
        let res = try!(check_status(self.client.get(&api_url!("album/".to_string() + &album_id))
            .headers(headers)
            .send())); 

        let json: Value = try!(json::from_reader(res));
        if let Some(images) = json.pointer("/data/images").and_then(|t| t.as_array()) {
            let all = images.iter().filter_map(|i| {
                i.pointer("/link").and_then(|l| l.as_str()).and_then(|s| Some(s.to_string()))
            }).collect::<Vec<String>>();
            Ok(all)
        } else {
            Err(ImgurError { kind: ImgurErrorKind::ResponseBodyNoImages} )
        }
    } 
}

#[derive(Debug)]
enum ImgurErrorKind {
    ResponseBodyNoImages,
    BadStatusCode(hyper::status::StatusCode),
    HyperError(hyper::Error),
    JsonError(json::error::Error),
}

#[derive(Debug)]
/// Error that can happen on image upload.
pub struct ImgurError {
    kind: ImgurErrorKind,
}


impl From<hyper::Error> for ImgurError {
    fn from(src: hyper::Error) -> Self {
        ImgurError { kind: ImgurErrorKind::HyperError(src) }
    }
}

impl From<json::error::Error> for ImgurError {
    fn from(src: json::error::Error) -> Self {
        ImgurError { kind: ImgurErrorKind::JsonError(src) }
    }
}

fn check_status(response: hyper::Result<hyper::client::Response>) -> Result<hyper::client::Response, ImgurError> {
    let response = try!(response);
    match response.status.is_success() {
        true => Ok(response),
        false => Err(ImgurError { kind: ImgurErrorKind::BadStatusCode(response.status) }),
    }
}
