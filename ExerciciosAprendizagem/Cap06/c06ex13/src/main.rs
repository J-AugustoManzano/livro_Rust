use std::io;
use std::io::prelude::*;

macro_rules! escreva {
    ($esc:expr) => (print!(concat!($esc, "\n")));
    ($esc:expr, $($param:tt)*) => (print!(concat!($esc, "\n"), $($param)*));
}

fn main() {

    escreva!("Estudo da linguagem Rust");

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
