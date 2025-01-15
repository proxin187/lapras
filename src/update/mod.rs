use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

const VERSION: usize = 11;


pub struct UpdateListener {
    ready: Arc<AtomicBool>,
    handle: JoinHandle<()>,
}

impl UpdateListener {
    pub fn new() -> UpdateListener {
        let ready = Arc::new(AtomicBool::new(false));
        let clone = ready.clone();

        let handle = thread::spawn(move || {
            while !clone.load(Ordering::Relaxed) {
                // TODO: we need some sort of endpoint to check the latest version and download a
                // new version, we dont want to leak our ip though, maybe we can use ngrok over
                // tor?

                thread::sleep(Duration::from_secs(5));
            }
        });

        UpdateListener {
            ready,
            handle,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    pub fn spawn(&self) {
        let ready = self.ready.clone();

    }
}


