# Example Usage
First make sure there's an image at the root of your directory. If it's called image.jpg here's how you can use image_captioner:
```
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

## About the BLIP Deep Learning Model

**BLIP (Bootstrapping Language-Image Pre-training)** is a deep learning model designed for unified vision-language understanding and generation. It is particularly effective in both understanding-based and generation-based vision-language tasks.

### Key Points:
- **Developed By:** Junnan Li, Dongxu Li, Caiming Xiong, and Steven Hoi.
- **Model Details:** For in-depth information, visit the [BLIP model card on Hugging Face](https://huggingface.co/Salesforce/blip-image-captioning-base).
- **License:** Released under the BSD-3-Clause License, allowing both personal and commercial use.
- **Commercial Use:** Suitable for commercial projects, but always review the [license terms](https://huggingface.co/Salesforce/blip-image-captioning-base/blob/main/LICENSE) for full compliance.

BLIP's architecture and capabilities make it a state-of-the-art choice for various vision-language tasks, including its ability to adapt to video-language tasks in a [zero-shot manner](https://towardsdatascience.com/understanding-zero-shot-learning-making-ml-more-human-4653ac35ccab).

## Image Path Clarification

When using `image_captioner`, the image path provided (e.g., `./image.jpg`) is relative to the directory from which you run your Rust application. This is typically the same directory where you execute the `cargo build` or `cargo run` commands.

### Example:
If your project structure is as follows:
/my_rust_project
/src
/target
image.jpg
Cargo.toml

And you run `cargo run` from `/my_rust_project`, then `./image.jpg` refers to the image located directly inside `/my_rust_project`.

### Note:
Ensure that the image file is present in the correct directory before running your application to avoid path-related errors.

## Model Download and Storage

### Initial Download
The first time you run `image_captioner`, it automatically downloads the BLIP model. This process requires an internet connection. The download may take some time due to the size of the model.

### Model Size
The BLIP model is approximately 990 MB in size. Ensure you have sufficient storage space available for the download.

### Download Location
The model is downloaded and cached by the `transformers` library. The default cache location is typically:
- **Linux/macOS:** `~/.cache/huggingface/hub`
- **Windows:** `C:\Users\<username>\.cache\huggingface\hub`

This location may vary based on your system configuration and environment settings. The `transformers` library manages the cache automatically, storing and retrieving the model for efficient usage in subsequent runs.

## Local Processing of Images

### Privacy and Security
`image_captioner` processes all images locally. The BLIP model runs directly on your machine, ensuring that your images are not sent to any external API or server. This local processing approach enhances privacy and security, as your data does not leave your device.

### Network Independence
Since the image processing is done locally, `image_captioner` does not require an internet connection after the initial model download. This makes it suitable for use in environments with limited or no network access, once the model is downloaded and set up.