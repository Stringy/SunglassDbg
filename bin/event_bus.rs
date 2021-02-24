use termion::event::Key;
use std::sync::{mpsc, Mutex};
use std::{thread, io};
use termion::input::TermRead;
use std::sync::mpsc::Sender;
use std::future::Future;

use lazy_static::lazy_static;
use std::time::Duration;

pub enum Event {
    Input(Key),
    Message(String),
    Tick,
}

pub struct Bus {
    tx: Mutex<mpsc::Sender<Event>>,
    rx: Mutex<mpsc::Receiver<Event>>,
    handles: Vec<thread::JoinHandle<()>>,
}

lazy_static! {
    static ref MAIN_BUS: Bus = Bus::new();
}

impl Bus {
    pub fn instance() -> &'static Self {
        &MAIN_BUS
    }

    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_secs(1);

        let handles = vec![
            Bus::input_thread(tx.clone()),
            // Bus::tick_thread(tx.clone(), tick_rate),
        ];

        Self {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
            handles,
        }
    }

    fn input_thread(tx: Sender<Event>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    if let Err(e) = tx.send(Event::Input(key)) {
                        eprintln!("{}", e);
                        break;
                    }
                }
            }
        })
    }

    fn tick_thread(tx: Sender<Event>, rate: Duration) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                if tx.send(Event::Tick).is_err() {
                    break;
                }
                thread::sleep(rate)
            }
        })
    }

    pub fn try_next(&self) -> Option<Event> {
        let rx = self.rx.lock().unwrap();
        rx.try_recv().ok()
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        let rx = self.rx.lock().unwrap();
        rx.recv()
    }

    pub fn post_message(&self, msg: &String) {
        let tx = self.tx.lock().unwrap();
        tx.send(Event::Message(msg.clone()));
    }

    pub fn post_tick(&self) {
        let tx = self.tx.lock().unwrap();
        tx.send(Event::Tick);
    }
}

