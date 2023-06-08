use std::io;
use std::io::prelude::*;

fn main() {

    let mut vlr = 10i32;
    let pb1 = &vlr as *const i32;
    let pb2 = &mut vlr as *mut i32;
    
    unsafe {
        println!("Ponteiro bruto 1: {}", *pb1);
        println!("Ponteiro bruto 2: {}", *pb2);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
