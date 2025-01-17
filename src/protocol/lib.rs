use serde::{Serialize, Deserialize};
use clap::Subcommand;


#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Message {
    Update {
        url: String,
    },
}


