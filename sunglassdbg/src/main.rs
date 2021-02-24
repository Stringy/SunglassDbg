#[cfg(feature = "use-tui")]
extern crate tui;

#[cfg(feature = "use-cli")]
extern crate cli;

use std::error::Error;

mod cmd;

#[cfg(feature = "use-tui")]
use tui::app::App;

#[cfg(feature = "use-cli")]
use cli::App;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();
    app.run()
}