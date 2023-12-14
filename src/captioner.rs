use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::io;
use std::fs;

/// Error type for captioning-related errors.
#[derive(Debug)]
pub enum CaptioningError {
    ImageNotFound,
    PythonExecutableNotFound,
    CommandExecutionFailed(io::Error),
    PythonScriptCopyFailed(std::io::Error), // Example variant with an associated error
    InvalidImagePath,

}

pub fn get_caption(image_path: &Path) -> Result<String, CaptioningError> {
    // Determine the location of the Python script within the target directory.
    let mut target_dir = PathBuf::from("./target");
    target_dir.push("image_captioner.py");

    // Check if the Python script exists in the target directory.
    if !target_dir.exists() {
        // If it doesn't exist, this is the first time it's being run.
        // Copy the script from your crate's assets to the target directory.
        let script_content = include_str!("../image_captioner.py");
        fs::write(&target_dir, script_content)
            .map_err(|e| CaptioningError::PythonScriptCopyFailed(e))?;
    }

    let python_executable = match which::which("python3") {
        Ok(path) => path,
        Err(_) => return Err(CaptioningError::PythonExecutableNotFound),
    };

    if !image_path.exists() {
        return Err(CaptioningError::ImageNotFound);
    }

    let image_path_str = image_path.to_str().ok_or(CaptioningError::InvalidImagePath)?;

    let output = Command::new(&python_executable)
        .arg(&target_dir)  // Use the script from the target directory.
        .arg(image_path_str)  // Pass the image path as a string.
        .stdout(Stdio::piped())  // Capture standard output
        .stderr(Stdio::piped())  // Capture standard error
        .output()
        .map_err(CaptioningError::CommandExecutionFailed)?;

    if output.status.success() {
        let caption = String::from_utf8_lossy(&output.stdout).to_string();
        println!("{}", String::from_utf8_lossy(&output.stderr));
        Ok(caption)
    } else {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        Err(CaptioningError::CommandExecutionFailed(io::Error::new(
            io::ErrorKind::Other,
            err,
        )))
    }
}
