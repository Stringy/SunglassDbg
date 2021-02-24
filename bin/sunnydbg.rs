extern crate rustyline;
extern crate sundbg;
extern crate tui;
extern crate unicode_width;
extern crate lazy_static;
extern crate log;

use std::error::Error;

mod event;
mod app;
mod event_bus;
mod logger;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(std::env::args().nth(1).unwrap());
    app.run()
}