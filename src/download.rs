use std::path::PathBuf;

use models::Image;

pub struct DownloadBuilder<'a>  {
    url: &'a str,
    path: PathBuf, 
}


impl <'a> DownloadBuilder<'a> {
    pub fn new(i: &Image) -> DownloadBuilder {
        DownloadBuilder {
            url: i.link.as_str(),     
            path: PathBuf::from("test"),
        }
    }
}
