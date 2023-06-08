use std::io;
use std::io::prelude::*;

fn main() {    

    let mut i: u16;
    let mut r: u16;

    i = 15;
    loop  {
        r = i.pow(2);
        println!("{:5}", r);
        i = i + 1;
        if i > 200 { break; }
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

