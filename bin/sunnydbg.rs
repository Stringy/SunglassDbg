extern crate sunglassdbg;
extern crate rustyline;

use sunglassdbg::process;
use std::env;

fn main() {
    println!("SunglassDbg v0.1");

    let p = process::start("/bin/ls", vec![], None).unwrap();
    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline("sunnyd>> ") {
        println!("{}", line);
    }
}