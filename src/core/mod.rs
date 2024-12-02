pub mod parser;

use std::sync::Arc;
use parking_lot::RwLock;
use crossbeam::channel;
use dashmap::DashMap;

pub struct Core {
    state: Arc<RwLock<State>>,
    cache: Arc<DashMap<String, Vec<u8>>>,
    tx: channel::Sender<Event>,
    rx: channel::Receiver<Event>,
}

struct State {
    running: bool,
    workers: usize,
}

#[derive(Debug)]
enum Event {
    Start,
    Stop,
    Error(String),
}

impl Core {
    pub fn new(workers: usize) -> Self {
        let (tx, rx) = channel::bounded(1024);
        Self {
            state: Arc::new(RwLock::new(State {
                running: false,
                workers,
            })),
            cache: Arc::new(DashMap::new()),
            tx,
            rx,
        }
    }

    pub fn start(&self) {
        let mut state = self.state.write();
        if !state.running {
            state.running = true;
            self.tx.send(Event::Start).unwrap();
        }
    }

    pub fn stop(&self) {
        let mut state = self.state.write();
        if state.running {
            state.running = false;
            self.tx.send(Event::Stop).unwrap();
        }
    }

    pub fn is_running(&self) -> bool {
        self.state.read().running
    }
}