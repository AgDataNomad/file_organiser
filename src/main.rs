use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::fs::{self, metadata};
use std::io;

fn main() {
    println!("Please enter the directory path:");
    let mut root_path = String::new();
    io::stdin()
        .read_line(&mut root_path)
        .expect("Failed to read input");
    let root_path = root_path.trim(); // Remove trailing newline

    println!("Please enter the file extension (e.g., txt, jpg, etc.):");
    let mut file_extension = String::new();
    io::stdin()
        .read_line(&mut file_extension)
        .expect("Failed to read input");
    let file_extension = file_extension.trim(); // Remove trailing newline

    let mut unique_folders: HashSet<String> = HashSet::new();

    for entry in walkdir::WalkDir::new(root_path)
        .min_depth(1)
        .max_depth(5)
        .follow_links(true)
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() {
                let folder_path = entry.path().to_string_lossy().to_string();
                if !folder_path.contains(".git") {
                    // Check if the folder contains files with the specified extension
                    if folder_contains_extension(&folder_path, file_extension) {
                        unique_folders.insert(folder_path);
                    }
                }
            }
        }
    }

    let mut sorted_folders: Vec<String> = unique_folders.into_iter().collect();
    sorted_folders.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    for folder_path in &sorted_folders {
        if let Ok(metadata) = metadata(folder_path) {
            if metadata.is_dir() {
                // Check if the folder contains any files
                if fs::read_dir(folder_path).is_ok() {
                    if let Ok(created) = metadata.created() {
                        let datetime: DateTime<Utc> = created.into();
                        let date_only = datetime.date_naive();
                        println!("{} - {:?}", date_only, folder_path);
                    } else {
                        println!("Folder: {} (Creation time not available)", folder_path);
                    }
                }
            }
        } else {
            println!("Error reading metadata for folder: {}", folder_path);
        }
    }
}

fn folder_contains_extension(folder_path: &str, extension: &str) -> bool {
    // Check if the folder contains files with the specified extension
    // You can customize this logic based on your requirements
    // For example, check if any file in the folder has the given extension
    // or if the folder itself has the extension (e.g., folder_name.ext)
    // Here, we check if any file in the folder ends with the specified extension
    walkdir::WalkDir::new(folder_path).into_iter().any(|entry| {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                if let Some(file_name) = entry.file_name().to_str() {
                    return file_name.ends_with(extension);
                }
            }
        }
        false
    })
}
