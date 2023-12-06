# Getting Started 
run `cargo build` to add rust dependencies

# Example Usage
First make sure there's an image at the root of your directory. If it's called image.jpg here's how you can use image_captioner:
```
extern crate image_captioner;

use image_captioner::get_caption;
use std::path::Path;

fn main() {
    let image_path = Path::new("./image.jpg"); // Convert the string to a Path
    match get_caption(image_path) {
        Ok(caption) => println!("Caption: {}", caption),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```