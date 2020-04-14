use std::io;
use std::io::Read;
use std::process;

extern crate fractran;
use fractran::{run, Program};

fn main() -> io::Result<()> {
    let mut inp = String::new();
    io::stdin().read_to_string(&mut inp)?;

    let mut prog: Program = match inp.trim().parse::<Program>() {
        Ok(p) => p,
        Err(msg) => {
            println!("{}", msg);
            process::exit(1)
        }
    };

    for n in run(&mut prog) {
        println!("{}", n);
    }

    Ok(())
}
