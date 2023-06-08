use std::io;
use std::io::prelude::*;

fn main() {

    let pa = Box::new(10);
    println!("Ponteiro [pa] = {}", *pa);
    println!("Endereço [pa] = {:p}", pa);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
