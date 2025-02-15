use std::collections::HashMap;

use reqwest::blocking::{Client, Response};
use reqwest::Error;
use log::info;

pub struct Discord {
    data: HashMap<String, String>,
    client: Client,
}

impl Discord {
    pub fn new() -> Discord {
        Discord {
            data: HashMap::new(),
            client: Client::new(),
        }
    }

    pub fn set_username(&mut self, username: &str) {
        self.data.insert(String::from("username"), username.to_string());
    }

    pub fn set_content(&mut self, content: String) {
        self.data.insert(String::from("content"), content);
    }

    pub fn get_ip(&self) -> String {
        self.client.get("https://api.ipify.org")
            .send()
            .map(|response| response.text().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn send(&self) -> Result<Response, Error> {
        info!("data: {:?}", self.data);

        self.client.post("https://discord.com/api/webhooks/1340040626066030593/xNvvE0EdKf4ozhZIWcJjIYTThDzUfMqBFi6Ry4yRONl_XHibSxjd9dFaEhj2xX9SAqgq")
            .json(&self.data)
            .send()
    }
}


