extern crate rustyline;
extern crate sundbg;
extern crate tui;
extern crate unicode_width;

use std::{env, io};
use std::error::Error;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::Terminal;

use sundbg::process;

use crate::event::{Event, Events};

mod event;
mod app;

use crate::app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();
    app.run()
}