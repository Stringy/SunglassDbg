use crate::error::Result;
use crate::process::Process;

pub struct Breakpoint {
    pub addr: u64,
    saved: u64,
    pub enabled: bool,
}

impl Breakpoint {
    pub fn new(addr: u64) -> Self {
        Self {
            addr,
            saved: 0,
            enabled: false,
        }
    }

    pub fn enable(&mut self, process: &Process) -> Result<()> {
        if self.enabled {
            return Ok(());
        }

        let bp_instruction = if cfg!(any(target_arch = "amd64", target_arch="x86_64")) {
            0xccu64
        } else {
            unimplemented!("invalid arch!");
        };

        self.saved = process.read(self.addr)? as u64;
        let word = (self.saved & !0xffu64) | bp_instruction;
        process.write(self.addr, word)?;
        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self, process: &Process) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        process.write(self.addr, self.saved)?;
        self.enabled = false;
        Ok(())
    }
}
