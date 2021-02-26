use crate::error::Result;
use crate::process::Process;

///
/// Encapsulates metadata about a breakpoint. It does not have ownership
/// over the process in which it is relevant, and that relationship
/// should be managed at a higher level (in the `Debugger`)
///
pub struct Breakpoint {
    /// The address of the breakpoint in the process, must be absolute.
    pub addr: u64,
    /// A single word from the process at the breakpoint address,
    /// to allow breakpoints to be enabled and disabled easily.
    saved: u64,
    /// Whether or not this breakpoint is enabled.
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

    ///
    /// Enables the breakpoint within the given process, recording the current
    /// word of text at that address, and then overwriting it with the breakpoint
    /// instruction (which varies per architecture)
    ///
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

    ///
    /// Disables the breakpoint in the traced process, by writing the saved
    /// word back into the process memory.
    ///
    pub fn disable(&mut self, process: &Process) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        process.write(self.addr, self.saved)?;
        self.enabled = false;
        Ok(())
    }
}
