use std::{fs, fs::File, path::Path, collections::HashMap, clone::Clone, error::Error};
use gpgme::{Context, Protocol};

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

#[derive(Clone, Debug)]
pub struct PasswordEntry {
    pub name: String,
    pub path: String,
}

// TODO maybe use https://crates.io/crates/secure-string?
#[derive(Debug)]
pub struct DecryptedPasswordEntry {
    pub name: String,
    pub password: String,
    pub properties: HashMap<String, String>,
    pub rest: String
}

pub struct PasswordStore {
    store_dir: String
}

impl Clone for PasswordStore {
    fn clone(&self) -> Self {
        PasswordStore {
            store_dir: self.store_dir.clone()
        }
    }
}

impl PasswordStore {
    pub fn new(store_dir: &str) -> PasswordStore {
        PasswordStore {
            store_dir: String::from(store_dir)
        }
    }

    pub fn list(&self, dir: &str) -> Result<Vec<PasswordItem>, String> {
        match fs::read_dir(Path::new(&self.store_dir).join(dir)) {
            Ok(paths) => Ok(paths
                .filter_map(|entry| -> Option<PasswordItem> {
                    match entry {
                        Ok(entry) => {
                            let path_buf = entry.path();

                            let name = path_buf.file_stem()?.to_str()?;
                            let path = path_buf.strip_prefix(&self.store_dir).ok()?.to_str()?;

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

    pub fn decrypt(&self, entry: &PasswordEntry) -> Result<DecryptedPasswordEntry, Box<dyn Error>> {
        let mut context = Context::from_protocol(Protocol::OpenPgp)?;

        let mut file = File::open(Path::new(&self.store_dir).join(&entry.path))?;
        let mut decrypted = Vec::new();

        context.decrypt(&mut file, &mut decrypted)?;

        let decrypted = String::from_utf8(decrypted)?;
        let mut lines = decrypted.lines();

        let password = lines.next().unwrap_or_default();

        let properties = lines
            .filter_map(|line| {
                let mut split = line.splitn(2, ':');
                let key = split.next()?;
                let value = split.next()?;
                Some((key.to_string(), value.to_string()))
            })
            .collect();

        Ok(DecryptedPasswordEntry {
            name: entry.name.clone(),
            password: password.to_string(),
            properties,
            rest: String::from(""), // TODO put all unparsed lines here
        })
    }
}
