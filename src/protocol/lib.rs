use serde::{Serialize, Deserialize};
use clap::Subcommand;

use std::net::{TcpStream, SocketAddr};
use std::io::{Write, Read};


#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Message {
    Update {
        url: String,
    },
}

pub struct Stream {
    stream: TcpStream,
}

impl Stream {
    pub fn new(stream: TcpStream) -> Stream {
        Stream {
            stream,
        }
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        self.stream.peer_addr().map_err(|err| err.into())
    }

    pub fn send(&mut self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = serde_json::to_vec(message)?;

        data.push(0x04);

        self.stream.write_all(&data)?;

        Ok(())
    }

    pub fn recv(&mut self) -> Result<Message, Box<dyn std::error::Error>> {
        let mut data: Vec<u8> = Vec::new();

        while !data.ends_with(&[0x04]) {
            let mut temp: [u8; 1024] = [0; 1024];

            let read = self.stream.read(&mut temp)?;

            data.extend(&temp[..read]);
        }

        Ok(serde_json::from_slice(&data[..data.len() - 1])?)
    }
}


