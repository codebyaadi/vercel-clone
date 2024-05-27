use std::path::{Path, PathBuf};

pub fn get_files_in_folder(folder_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            // Recursively call get_files_in_folder for nested directories
            if let Ok(mut nested_files) = get_files_in_folder(&path) {
                files.append(&mut nested_files);
            }
        }
    }
    Ok(files)
}
