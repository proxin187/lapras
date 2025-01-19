use serde::{Serialize, Deserialize};
use clap::Subcommand;

use std::net::{TcpStream, SocketAddr, Shutdown};
use std::io::{Write, Read};
use std::cell::RefCell;


#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Message {
    Update {
        url: String,
    },
    Ping,
}

pub struct Stream {
    stream: RefCell<TcpStream>,
}

impl Stream {
    pub fn new(stream: TcpStream) -> Stream {
        Stream {
            stream: RefCell::new(stream),
        }
    }

    pub fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.stream.borrow().shutdown(Shutdown::Both).map_err(|err| err.into())
    }

    pub fn peer_addr(&self) -> Result<SocketAddr, Box<dyn std::error::Error>> {
        self.stream.borrow().peer_addr().map_err(|err| err.into())
    }

    pub fn send(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let mut data = serde_json::to_vec(message)?;

        data.push(0x04);

        self.stream.borrow_mut().write_all(&data)?;

        Ok(())
    }

    pub fn recv(&self) -> Result<Message, Box<dyn std::error::Error>> {
        let mut data: Vec<u8> = Vec::new();

        while !data.ends_with(&[0x04]) {
            let mut temp: [u8; 1024] = [0; 1024];

            match self.stream.borrow_mut().read(&mut temp)? {
                0 => return Err("unexpected eof".into()),
                n => data.extend(&temp[..n]),
            }
        }

        Ok(serde_json::from_slice(&data[..data.len() - 1])?)
    }
}


