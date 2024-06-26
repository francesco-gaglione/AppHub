use std::{env, fs};
use std::path::{Path, PathBuf};
use log::{error};
use crate::desktop_entry::DesktopEntry;
use crate::desktop_file_builder::DesktopFileBuilder;

/// Find the desktop entry of the application with the given name.
/// The function reads all the .desktop files in the applications directory and compares the "Name" value
/// of each file with the given app_name. If a match is found, the function returns the DesktopEntry struct
pub fn find_desktop_entry(app_name: String) -> Result<DesktopEntry, String> {
    // read all .desktop files in the applications directory
    let applications_dir: PathBuf = find_desktop_file_location().map_err(|e| e.to_string())?;
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };

                let entry_path = entry.path();
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(&entry_path, true);

                match desktop_file {
                    Ok(desktop_entry) => {
                        if desktop_entry.name().unwrap() == app_name {
                            return Ok(DesktopEntry {
                                exec: desktop_entry.exec().unwrap(),
                                name: desktop_entry.name().unwrap(),
                                icon: desktop_entry.icon().unwrap(),
                            });
                        }
                    }
                    Err(err) => {
                        error!("Failed to read desktop file: {}", err);
                        continue;
                    }
                }
            }
            return Err(format!("App not found: {}", app_name));
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }
}

/// Find the desktop entries containing the given string in the "Exec" value.
/// The function reads all the .desktop files in the applications directory and compares the "Exec" value
/// of each file with the given contains_exec. If a match is found, the function returns a vector of paths
/// to the .desktop files.
pub fn find_desktop_entries_by_exec_contains(contains_exec: &String) -> Result<Vec<String>, String> {
    let mut desktop_entries_paths: Vec<String> = Vec::new();

    // read all .desktop files in the applications directory
    let applications_dir: PathBuf = find_desktop_file_location().map_err(|e| e.to_string())?;
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };

                let entry_path = entry.path();
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(&entry_path, true);

                match desktop_file {
                    Ok(desktop_entry) => {
                        if desktop_entry.exec().unwrap().contains(contains_exec) {
                            desktop_entries_paths.push(entry_path.to_string_lossy().to_string());
                        }
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(desktop_entries_paths)
}

/// Delete the desktop file of the application with the given name.
/// The function returns true if the file is successfully deleted, and false otherwise.
/// If the file is not found, the function returns an error message.
/// The function reads all the .desktop files in the applications directory and compares the "Name" value
/// of each file with the given app_name. If a match is found, the file is deleted.
pub fn delete_desktop_file_by_name(app_name: &String) -> Result<bool, String> {
    let applications_dir: PathBuf = find_desktop_file_location().map_err(|e| e.to_string())?;
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {

                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };

                let entry_path = entry.path();
                let path_str = entry_path.to_str().ok_or("Failed to convert path to string")?;
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(&entry_path, true);

                match desktop_file {
                    Ok(desktop_entry) => {
                        match desktop_entry.name() {
                            Some(name) => {
                                if name == *app_name {
                                    match fs::remove_file(path_str) {
                                        Ok(_res) => {
                                            return Ok(true);
                                        }
                                        Err(error) => {
                                            return Err(format!("Failed to remove file: {}", error));
                                        }
                                    }
                                }
                            }
                            None => {
                                error!("Failed to read desktop file: Name is None");
                                continue;
                            }
                        }
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }
            }
            return Err(format!("App not found: {}", app_name));
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }
}

pub fn find_desktop_file_location() -> Result<PathBuf, &'static str> {
    // Check the KDE environment variable
    if let Ok(path) = env::var("KDE_INSTALL_APPDIR") {
        return Ok(PathBuf::from(path));
    }

    // Check the standard directories
    let global_path = "/usr/share/applications";
    if Path::new(global_path).exists() {
        return Ok(PathBuf::from(global_path));
    }

    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let local_path = home_dir.join(".local").join("share").join("applications");
    if Path::new(&local_path).exists() {
        return Ok(local_path);
    }

    // If neither environment variable is set, use a default path
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let default_path = home_dir.join(".local").join("share").join("applications");

    if Path::new(&default_path).exists() {
        Ok(default_path)
    } else {
        Err("Default path does not exist")
    }
}


// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_desktop_file_location() {
        let result = find_desktop_file_location().unwrap();
        println!("Desktop file location: {}", result);
        assert!(!result.is_empty());
    }
}