use std::{fs, path};

#[derive(Debug)]
pub enum PasswordItem {
    Directory(PasswordDirectory),
    Entry(PasswordEntry),
}

#[derive(Debug)]
pub struct PasswordDirectory {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct PasswordEntry {
    pub name: String,
    pub path: String,
}

pub fn list(store_dir: &str, dir: &str) -> Result<Vec<PasswordItem>, String> {
    match fs::read_dir(path::Path::new(store_dir).join(dir)) {
        Ok(paths) => Ok(paths
            .filter_map(|entry| -> Option<PasswordItem> {
                match entry {
                    Ok(entry) => {
                        let path_buf = entry.path();

                        let name = path_buf.file_stem()?.to_str()?;
                        let path = path_buf.strip_prefix(store_dir).ok()?.to_str()?;

                        if name == ".gpg-id" {
                            return None;
                        }

                        if path.starts_with(".git") {
                            return None;
                        }

                        let name = if name.ends_with(".gpg") {
                            &name[..name.len() - 4]
                        } else {
                            name
                        };

                        match path_buf.is_dir() {
                            true => Some(PasswordItem::Directory(PasswordDirectory {
                                name: String::from(name),
                                path: String::from(path),
                            })),
                            false => Some(PasswordItem::Entry(PasswordEntry {
                                name: String::from(name),
                                path: String::from(path),
                            })),
                        }
                    }
                    Err(_) => None,
                }
            })
            .collect()),
        Err(_) => Err(String::from("Error reading passwords")),
    }
}
