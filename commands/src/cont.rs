use crate::Command;
use debug::Debugger;
use std::error::Error;

use clap::Clap;

#[derive(Clap)]
pub struct ContinueCommand;

impl Command for ContinueCommand {
    fn run(&self, dbg: &mut Debugger) -> Result<(), Box<dyn Error>> {
        dbg.proceed();
        Ok(())
    }
}

