# Getting Started 
run `cargo build` to add rust dependencies
run `cargo run --bin setup_python` to set up python venv and dependencies

# Example Usage
```
extern crate rust_image_captioner;


use rust_image_captioner::get_caption;

fn main() {
    let image_path = "./image.jpg"; // Replace with actual image path

    match get_caption(image_path) {
        Ok(caption) => println!("Caption: {}", caption),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```