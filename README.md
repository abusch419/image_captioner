# Getting Started 
run `cargo build` to add rust dependencies

# Example Usage
```
extern crate rust_image_captioner;

use rust_image_captioner::get_caption;
use std::path::Path;

fn main() {
    let image_path = Path::new("./image.jpg"); // Convert the string to a Path
    match get_caption(image_path) {
        Ok(caption) => println!("Caption: {}", caption),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```