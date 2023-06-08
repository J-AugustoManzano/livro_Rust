use std::io;
use std::io::prelude::*;

fn main() {    

    let mut s: i64;

    s = 0;
    for i in 1 .. 101 {
		s += i as i64;
    }
    println!("{}", s);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
