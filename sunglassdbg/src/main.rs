#[cfg(feature = "use-tui")]
extern crate tui;

#[cfg(feature = "use-cli")]
extern crate cli;

use std::error::Error;

#[cfg(feature = "use-cli")]
use cli::App;
#[cfg(feature = "use-tui")]
use tui::app::App;

mod cmd;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new();
    app.run()
}