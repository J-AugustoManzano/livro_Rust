use std::io;
use std::io::prelude::*;

fn escreva1(cadeia: String) {
    println!("{}", cadeia);
}

fn escreva2(cadeia: &str) {
    println!("{}", cadeia);
}

fn main() {

    escreva1("Alô, Mundo!".to_string());
    println!();

    escreva2("Alô, Mundo!");

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
