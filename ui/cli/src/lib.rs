extern crate rustyline;

use std::error::Error;
use rustyline::{Config, CompletionType, EditMode, Editor, KeyEvent, Cmd};
use common::cli::{CommandLine, Clap};
use debug::Debugger;

const PROMPT: &'static str = "sdbg>> ";

pub struct App {
    history_file: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            history_file: String::from(".sdbg_history")
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let cmdline = CommandLine::parse_from(std::env::args());
        let debugger = Debugger::from_config(cmdline);

        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Vi)
            .build();

        let mut rl: Editor<()> = Editor::with_config(config);
        rl.load_history(self.history_file.as_str());
        rl.bind_sequence(KeyEvent::ctrl('r'), Cmd::HistorySearchForward);
        rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::ClearScreen);

        loop {
            match rl.readline(PROMPT) {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(line.as_str());
                }
                Err(e) => {
                    use rustyline::error::ReadlineError::*;
                    match e {
                        Eof | Interrupted => { break; }
                        _ => eprintln!("input error: {:?}", e)
                    };
                }
            }
        }

        rl.append_history(self.history_file.as_str());
        Ok(())
    }
}