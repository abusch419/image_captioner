use std::process::Command;
use std::env;
use which::which;

fn main() {
    // Check if Python is installed
    if let Ok(python_exe) = which("python3") {
        println!("Found Python 3 executable at: {:?}", python_exe);

        // Check for a virtual environment
        if env::var("VIRTUAL_ENV").is_err() {
            println!("No virtual environment found. Creating one...");
            Command::new("python3")
                .arg("-m")
                .arg("venv")
                .arg("venv")
                .status()
                .expect("Failed to create a virtual environment.");
        }

        // Activate the virtual environment
        env::set_var("VIRTUAL_ENV", "venv");

        // Install Python dependencies
        install_python_dependencies();
    } else {
        eprintln!("Python 3 executable not found. Please ensure Python 3 is installed and in your PATH.");
        std::process::exit(1);
    }
}

fn install_python_dependencies() {
    let status = Command::new("pip")
        .arg("install")
        .arg("torch")
        .arg("torchvision")
        .arg("torchaudio")
        .arg("requests")
        .arg("transformers")
        .arg("Pillow")
        .status()
        .expect("Failed to install Python dependencies.");

    if status.success() {
        println!("Python dependencies installed successfully.");
    } else {
        eprintln!("Failed to install Python dependencies.");
        std::process::exit(1);
    }
}
