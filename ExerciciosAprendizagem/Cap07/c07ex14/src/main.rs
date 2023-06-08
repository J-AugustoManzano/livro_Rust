use std::io;
use std::io::prelude::*;

fn main() {

    let mut faixa = 1..6;

    loop {
        match faixa.next() {
            Some(i) => {
                println!("{}", i);
            },
            None => break
        }
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
