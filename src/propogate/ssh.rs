use std::process::Command;
use std::fs;

// TODO: maybe we should consider just having a script do this?


pub struct Session {
}

impl Session {
    pub fn connect() {
    }
}

pub struct Ssh {
    keys: Vec<String>,
}

impl Ssh {
    pub fn new() -> Ssh {
        Ssh {
            keys: Vec::new(),
        }
    }

    pub fn find_keys(&mut self) {
    }
}


