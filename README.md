# imgur-rs
[![docs](https://docs.rs/imgur_rs/badge.svg)](https://docs.rs/imgur_rs)

## Example

```rust
extern crate imgur_rs as imgur;

use std::fs::File;

use imgur::{Image, ImgurHandle, ProvidesFile};

fn download_image(h: &ImgurHandle, i: &Image) {
    let path = format!("./images/{}", i.filename());
    let mut file = File::create(path).expect("file couldnt be created");
    h.download_image(i, &mut file)
        .expect("failed to download image");
}

fn main() {
    let client_id = "some-client-id";
    let handle = ImgurHandle::new(client_id.to_string());

    if let Ok(images) = handle.get_album("4esDv").map(|r| r.data.images) {
        images.iter().for_each(|&ref i| download_image(&handle, i));
    }
}

```
