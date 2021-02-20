extern crate sundbg;
extern crate rustyline;

use sundbg::process;
use std::env;

fn main() {
    println!("SunglassDbg v0.1");

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline("sunnyd>> ") {
        if !line.is_empty() {
            println!("{}", line);
        }
    }
}