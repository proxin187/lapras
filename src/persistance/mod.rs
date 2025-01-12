use std::process::Command;
use std::io::Write;


pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    let mut process = Command::new("/usr/bin/crontab").spawn()?;

    if let Some(stdin) = process.stdin.take() {
        // TODO: crontab pipe
    }

    Ok(())
}


