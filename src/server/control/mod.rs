use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

macro_rules! lock {
    ($mutex:expr) => {
        $mutex.lock().map_err(|_| Into::<Box<dyn std::error::Error>>::into("failed to lock"))
    }
}

pub struct Controller {
    clients: Arc<Mutex<Vec<TcpStream>>>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn run(&mut self) {
    }
}

fn listen(clients: Arc<Mutex<Vec<TcpStream>>>) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8787")?;

    thread::spawn(move || {
        for incoming in listener.incoming() {
            if let Ok(stream) = incoming {
                let _ = lock!(clients).map(|mut clients| clients.push(stream));
            }
        }
    });

    Ok(())
}


