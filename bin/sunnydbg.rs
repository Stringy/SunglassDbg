extern crate rustyline;
extern crate sundbg;
extern crate tui;
extern crate unicode_width;

use std::error::Error;

mod event;
mod app;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(std::env::args().nth(1).unwrap());
    app.run()
}