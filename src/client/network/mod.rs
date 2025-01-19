use std::net::TcpStream;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use protocol::{Message, Stream};
use log::{info, warn};


pub struct Network {
    stream: Option<Stream>,
    delay: Duration,
    should_close: Arc<AtomicBool>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            stream: None,
            delay: Duration::from_secs(2),
            should_close: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn should_close(&self) -> Arc<AtomicBool> {
        self.should_close.clone()
    }

    fn try_connect(&mut self) {
        match TcpStream::connect("127.0.0.1:8080") {
            Ok(stream) => {
                info!("successfully connected");

                self.stream.replace(Stream::new(stream));
            },
            Err(err) => {
                warn!("failed to connect: {}, trying again", err);

                thread::sleep(self.delay);
            },
        }
    }

    fn handle_message(&mut self, message: Message) {
        info!("message: {:?}", message);

        match message {
            Message::Update { url } => {
                self.should_close.store(true, Ordering::Relaxed);
            },
            Message::Ping => {
                info!("recieved ping");
            },
        }
    }

    fn shutdown(&mut self) {
        if let Some(stream) = &mut self.stream {
            let _ = stream.shutdown();
        }
    }

    pub fn run(&mut self) {
        while !self.should_close.load(Ordering::Relaxed) {
            match &mut self.stream {
                Some(stream) => match stream.recv() {
                    Ok(message) => {
                        self.handle_message(message);
                    },
                    Err(err) => {
                        warn!("lost connection: {}", err);

                        self.stream.take();
                    },
                },
                None => {
                    self.try_connect();
                },
            }
        }

        self.shutdown()
    }
}


