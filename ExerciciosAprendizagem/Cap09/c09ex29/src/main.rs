use std::io;
use std::io::prelude::*;

fn main() {

    let alo: &str = "Alô,";
    let mundo: &str = " Mundo!";

    let cadeia: String = String::new() + alo + mundo;

    println!("{}", cadeia);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
