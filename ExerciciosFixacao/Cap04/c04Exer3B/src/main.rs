use std::io;
use std::io::prelude::*;

fn main() {    

    let mut i: i64;
    let mut s: i64;

    s = 0;
    i = 1;
    loop {
		s += i;
        i += 1;
        if i > 100 { break; }
    }
    println!("{}", s);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
