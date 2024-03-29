use std::collections::HashSet;
use walkdir::WalkDir;
use std::fs::metadata; // Import the metadata function
use chrono::{DateTime, Utc};


fn main() {
    let root_path = "A:/"; // Replace with your actual directory path

    let mut unique_folders: HashSet<String> = HashSet::new(); // Use HashSet<String> for folder paths

    // Recursively walk the directory and collect unique folder paths (excluding root)
    for entry in WalkDir::new(root_path)
        .min_depth(1)
        .max_depth(5)
        .follow_links(true) {
        if let Ok(entry) = entry {
            if entry.file_type().is_dir() {
                let folder_path = entry.path().to_string_lossy().to_string();
                if !folder_path.contains(".git") {
                    unique_folders.insert(folder_path);
                }
            }
        }
    }

    // Sort the unique folder paths in ascending order
    let mut sorted_folders: Vec<String> = unique_folders.into_iter().collect();
    sorted_folders.sort();

    // Now you can process the sorted folder paths:
    for folder_path in &sorted_folders {
        // Retrieve creation timestamp (platform-specific)
        if let Ok(metadata) = metadata(folder_path) {
            if let Ok(created) = metadata.created() {
                let datetime: DateTime<Utc> = created.into();
                let date_only = datetime.date_naive();
                println!("{} - {:?}", date_only, folder_path);
            } else {
                println!("Folder: {} (Creation time not available)", folder_path);
            }
        } else {
            println!("Error reading metadata for folder: {}", folder_path);
        }
    }
}