use std::error::Error;

use clap::Clap;

use debug::Debugger;

use crate::Command;

#[derive(Clap)]
pub struct RunCommand {
    #[clap(short = 'a')]
    args: Vec<String>
}

impl Command for RunCommand {
    fn run(&self, dbg: &mut Debugger) -> Result<(), Box<dyn Error>> {
        dbg.run(self.args.clone(), vec![])?;
        Ok(())
    }
}