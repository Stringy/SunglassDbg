mod cont;
mod run;

extern crate clap;
extern crate debug;

use clap::Clap;

use log::error;

use debug::Debugger;
use std::error::Error;

pub trait Command {
    fn run(&self, dbg: &mut Debugger) -> Result<(), Box<dyn Error>>;
}

use clap::AppSettings;

#[derive(Clap)]
#[clap(
setting = AppSettings::SubcommandRequired,
setting = AppSettings::NoBinaryName
)]
pub struct Commands {
    #[clap(subcommand)]
    pub cmd: Sub
}

#[derive(Clap)]
pub enum Sub {
    #[clap(version = "1.0")]
    Cont(cont::ContinueCommand),
    Run(run::RunCommand),
}

impl Commands {
    pub fn parse_line(line: String) -> Option<Box<dyn Command>> {
        match Commands::try_parse_from(line.split_whitespace()) {
            Ok(commands) => {
                use Sub::*;
                match commands.cmd {
                    Cont(c) => Some(Box::new(c)),
                    Run(c) => Some(Box::new(c))
                }
            }
            Err(e) => {
                error!("{}", e);
                None
            }
        }
    }
}