mod control;

use control::Controller;

use env_logger::{Builder, Env};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::from_env(Env::default().default_filter_or("info")).init();

    let controller = Controller::new();

    controller.run()
}


