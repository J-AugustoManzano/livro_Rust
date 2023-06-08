use std::io;
use std::io::prelude::*;

fn main() {    

    for i in 1 .. 200 {
		if i as i64 % 4 != 0 {
		    println!("{:3}", i);
		}
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
