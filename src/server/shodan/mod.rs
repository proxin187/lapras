use reqwest::blocking::Client;
use serde::Deserialize;
use log::{info, warn};

use std::time::Duration;


#[derive(Debug, Deserialize)]
pub struct Host {
    pub ip: u32,
}

#[derive(Debug, Deserialize)]
pub struct Search {
    #[serde(rename = "matches")]
    pub hosts: Vec<Host>,

    pub total: usize,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Success {
        #[serde(flatten)]
        search: Search,
    },
    Error {
        error: String,
    },
}

pub struct Shodan {
    client: Client,
    key: String,
}

impl Shodan {
    pub fn new(key: &str) -> Shodan {
        Shodan {
            client: Client::new(),
            key: key.to_string(),
        }
    }

    pub fn search(&self, query: &str, page: &mut usize) -> Option<Search> {
        loop {
            let url = format!("https://api.shodan.io/shodan/host/search?key={}&query={}&page={}&facets=country", self.key, query, *page);

            match self.client.get(&url).timeout(Duration::from_secs(60)).send().and_then(|response| response.json()) {
                Ok(response) => match response {
                    Response::Success { search } => {
                        info!("search successful");

                        return Some(search);
                    },
                    Response::Error { error } => {
                        warn!("reached end of search: {}", error);

                        return None;
                    },
                },
                Err(err) => {
                    warn!("reqwest failed with `{}`, trying next", err);

                    *page += 1;
                },
            }
        }
    }
}


