use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::thread;

use protocol::Message;
use rustyline::DefaultEditor;
use log::{info, warn};
use clap::{Parser, Subcommand};

macro_rules! lock {
    ($mutex:expr) => {
        $mutex.lock().map_err(|_| Into::<Box<dyn std::error::Error>>::into("failed to lock"))
    }
}

#[derive(Debug, Parser)]
#[command(multicall = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Message {
        #[command(subcommand)]
        message: Message,
    },
    List,
}

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client {
            stream,
        }
    }

    pub fn send(&mut self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = serde_json::to_vec(message)?;

        self.stream.write_all(&bytes).map_err(|err| err.into())
    }
}

pub struct Controller {
    clients: Arc<Mutex<Vec<Client>>>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn command(&self, cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
        match cli.command {
            Command::Message { message } => {
                info!("sending message to all clients");

                for client in lock!(self.clients)?.iter_mut() {
                    if let Err(err) = client.send(&message) {
                        warn!("failed to send {}, {}", client.stream.peer_addr()?, err);
                    }
                }

                info!("done");
            },
            Command::List => {
                let lock = lock!(self.clients)?;

                for client in lock.iter() {
                    info!("addr: {}", client.stream.peer_addr()?);
                }

                info!("total {} clients", lock.len());
            },
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        listen(self.clients.clone())?;

        let mut rl = DefaultEditor::new()?;

        loop {
            let readline = rl.readline("[lapras]$ ");

            match readline {
                Ok(line) => {
                    let result = shlex::split(&line)
                        .ok_or(Into::<Box<dyn std::error::Error>>::into("failed to lex"))
                        .and_then(|tokens| Cli::try_parse_from(tokens).map_err(|err| err.into()));

                    match result {
                        Ok(cli) => {
                            self.command(cli)?;
                        },
                        Err(err) => {
                            println!("{}", err);
                        },
                    }
                },
                Err(err) => {
                    warn!("failed to readline: {}", err);

                    break;
                },
            }
        }

        Ok(())
    }
}

fn listen(clients: Arc<Mutex<Vec<Client>>>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8787")?;

    thread::spawn(move || {
        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let client = Client::new(stream);

                let _ = lock!(clients).map(|mut clients| clients.push(client));
            }
        }
    });

    Ok(())
}


