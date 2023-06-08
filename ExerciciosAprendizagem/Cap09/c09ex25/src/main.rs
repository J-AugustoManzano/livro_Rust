#![allow(unused_assignments)]
use std::io;
use std::io::prelude::*;

fn main() {

    let alo: &'static str = "Alô,";
    let mundo: &'static str = " Mundo!";
    
    let mut cadeia1: String = String::new();
    let mut cadeia2: String = String::new();

    cadeia1 += alo;
    cadeia1 += mundo;

    println!("Cadeia 1 = {}", cadeia1);
    println!();

    cadeia2 = alo.to_string() + mundo;

    println!("Cadeia 2 = {}", cadeia2);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
