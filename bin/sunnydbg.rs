extern crate rustyline;
extern crate sundbg;
extern crate tui;

use std::error::Error;

mod cmd;

use tui::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new(std::env::args().nth(1).unwrap());
    app.run()
}