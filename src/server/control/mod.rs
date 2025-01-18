use std::sync::{Arc, Mutex};
use std::net::TcpListener;
use std::thread;

use protocol::{Message, Stream};
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

pub struct Controller {
    clients: Arc<Mutex<Vec<Stream>>>,
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
                        warn!("failed to send {}, {}", client.peer_addr()?, err);
                    }
                }

                info!("done");
            },
            Command::List => {
                let lock = lock!(self.clients)?;

                for client in lock.iter() {
                    info!("addr: {}", client.peer_addr()?);
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

fn listen(clients: Arc<Mutex<Vec<Stream>>>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    thread::spawn(move || {
        for incoming in listener.incoming() {
            if let Ok((stream, mut clients)) = incoming.map_err(|err| err.into()).and_then(|incoming| lock!(clients).map(|clients| (incoming, clients))) {
                let stream = Stream::new(stream);

                if !clients.contains(&stream) {
                    clients.push(stream);
                }
            }
        }
    });

    Ok(())
}


