use std::thread::{self, JoinHandle};
use std::net::TcpListener;


pub fn lock() -> Option<JoinHandle<()>> {
    match TcpListener::bind("127.0.0.1:32887") {
        Ok(listener) => {
            let handle = thread::spawn(move || {
                for _ in listener.incoming() {}
            });

            Some(handle)
        },
        Err(_) => None,
    }
}


