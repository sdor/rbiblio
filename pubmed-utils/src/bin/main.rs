use std::env;

pub fn retrieve_current_directory() {
    // Attempt to retrieve the current directory
    if let Ok(current_dir) = env::current_dir() {
        // Print the current directory path
        println!("Current directory: {}", current_dir.display());
    } else {
        // Handle the case where retrieving the current directory fails
        eprintln!("Failed to retrieve current directory");
    }
}

fn main() {
    retrieve_current_directory();
}