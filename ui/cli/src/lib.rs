extern crate commands;
extern crate debug;
extern crate log;
extern crate rustyline;

use std::error::Error;

use rustyline::{Cmd, CompletionType, Config, EditMode, Editor, KeyEvent};

use commands::Commands;
use common::cli::{Clap, CommandLine};
use debug::Debugger;
use std::io::Write;

mod logger;

const PROMPT: &'static str = "sdbg>> ";

pub struct App {
    history_file: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            history_file: String::from(".sdbg_history")
        }
    }
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        logger::init_logger().unwrap();

        let cmdline = CommandLine::parse_from(std::env::args());
        let mut debugger = Debugger::from_config(cmdline);

        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::List)
            .edit_mode(EditMode::Vi)
            .build();

        let mut rl: Editor<()> = Editor::with_config(config);
        rl.load_history(self.history_file.as_str())?;
        rl.bind_sequence(KeyEvent::ctrl('r'), Cmd::HistorySearchForward);
        rl.bind_sequence(KeyEvent::ctrl('l'), Cmd::ClearScreen);

        loop {
            match rl.readline(PROMPT) {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }
                    rl.add_history_entry(line.as_str());

                    if line == "exit" {
                        match self.handle_exit(&mut debugger) {
                            Ok(should_exit) => {
                                if should_exit {
                                    break;
                                }
                            }
                            Err(e) => {
                                eprintln!("{}", e);
                            }
                        }
                    }

                    let error = match Commands::parse_line(line) {
                        Some(cmd) => cmd.run(&mut debugger),
                        None => Ok(())
                    };

                    if let Err(error) = error {
                        eprintln!("{}", error);
                    }
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

        rl.append_history(self.history_file.as_str())?;
        Ok(())
    }

    fn handle_exit(&self, dbg: &mut Debugger) -> Result<bool, Box<dyn Error>> {
        if dbg.process_is_running() {
            let mut confirm = String::new();

            loop {
                print!("process is still running! detach? (Y/n) ");
                confirm.clear();
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut confirm)?;
                let input = confirm.strip_suffix("\n").unwrap_or_default();
                match input {
                    "Y" | "y" | "yes" => {
                        dbg.detach()?;
                        return Ok(true);
                    }
                    "N" | "n" | "no" => {
                        return Ok(false);
                    }
                    _ => continue
                };
            }
        }
        Ok(true)
    }
}
