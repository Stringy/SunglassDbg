use clap::Clap;
use crate::Command;
use debug::Debugger;
use std::error::Error;

#[derive(Clap)]
pub struct BreakCommand {
    #[clap(subcommand)]
    cmd: BreakSubCommand
}

#[derive(Clap)]
pub enum BreakSubCommand {
    New { addr: u64 },
    Remove { which: usize },
    Enable { which: usize },
    Disable { which: usize },
    List,
}

impl Command for BreakCommand {
    fn run(&self, dbg: &mut Debugger) -> Result<(), Box<dyn Error>> {
        match self.cmd {
            BreakSubCommand::New { addr } => dbg.add_breakpoint(addr)?,
            BreakSubCommand::Remove { which } => dbg.remove_breakpoint(which)?,
            BreakSubCommand::List => dbg.list_breakpoints(),
            BreakSubCommand::Enable { which } => dbg.enable_breakpoint(which)?,
            BreakSubCommand::Disable { which } => dbg.enable_breakpoint(which)?
        };
        Ok(())
    }
}
