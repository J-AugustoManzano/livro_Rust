use std::io;
use std::io::prelude::*;

fn main() {    

    let mut i: i64;

    i = 1;
    while !(i > 199) {
		if i % 4 == 0 {
		    println!("{:3}", i);
		}
        i += 1;
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
