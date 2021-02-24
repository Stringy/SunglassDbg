
pub use clap::Clap;

#[derive(Clap)]
#[clap(name = "SunglassDbg", version = "1.0", author = "@Giles")]
pub struct CommandLine {
    #[clap(short, long, about = "The path to the file to debug.")]
    pub file: Option<String>,

    #[clap(short, long, about = "The PID of the process to attach to.")]
    pub pid: Option<i32>,
}
