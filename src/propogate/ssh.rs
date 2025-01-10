use std::process::Command;
use std::path::PathBuf;
use std::fs;


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
        let rsa_id = ["/root/id_rsa*", "/root/**/id_rsa*", "/root/**/**/id_rsa*"].iter()
            .filter_map(|wildcard| {
                glob::glob(&wildcard)
                    .ok()
                    .map(|paths| paths.filter_map(|entry| entry.ok()).collect::<Vec<PathBuf>>())
            })
            .flatten()
            .collect::<Vec<PathBuf>>();

        // TODO: detect more keys

        Ssh {
            keys: [rsa_id].concat(),
        }
    }
}


