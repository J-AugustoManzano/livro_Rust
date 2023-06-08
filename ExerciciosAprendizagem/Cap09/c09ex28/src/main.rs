use std::io;
use std::io::prelude::*;

fn main() {

    let alo = String::from("Alô");
    let mundo = String::from(" Mundo!");

    let mut cadeia = String::new();

    cadeia.push_str(&alo);
    cadeia.push(','); 
    cadeia.push_str(mundo.as_str());

    println!("{}", cadeia);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
