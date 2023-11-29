// src/captioner.rs

use std::path::Path;
use which::which;
use std::process::Command;
use std::io;

/// Error type for captioning-related errors.
#[derive(Debug)]
pub enum CaptioningError {
    ImageNotFound,
    PythonExecutableNotFound,
    CommandExecutionFailed(io::Error),
}

/// Get a caption for an image.
///
/// # Arguments
///
/// * `image_path` - A string slice representing the path to the image.
///
/// # Returns
///
/// * `Result<String, CaptioningError>` - A `Result` containing the caption or an error.
pub fn get_caption(image_path: &str) -> Result<String, CaptioningError> {
    let python_script = "./image_captioner.py";
    let python_executable = match which("python3") {
        Ok(path) => path,
        Err(_) => return Err(CaptioningError::PythonExecutableNotFound),
    };

    if !Path::new(image_path).exists() {
        return Err(CaptioningError::ImageNotFound);
    }

    let output = Command::new(&python_executable)
        .arg(python_script)
        .arg(image_path)
        .output()
        .map_err(CaptioningError::CommandExecutionFailed)?;

    if output.status.success() {
        let caption = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(caption)
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(CaptioningError::CommandExecutionFailed(io::Error::new(
            io::ErrorKind::Other,
            err,
        )))
    }
}