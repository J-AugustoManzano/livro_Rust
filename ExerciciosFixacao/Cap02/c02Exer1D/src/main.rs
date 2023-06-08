use std::io;
use std::io::prelude::*;

fn main() {    

    let mut c = String::new();
    let mut l = String::new();
    let mut a = String::new();

    let comprimento: f32;
    let largura: f32;
    let altura: f32;
    let volume: f32;
 
    print!("Entre o valor do comprimento ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c).unwrap();
    comprimento = c.trim().parse::<f32>().unwrap();

    print!("Entre o valor da largura ......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut l).unwrap();
    largura = l.trim().parse::<f32>().unwrap();

    print!("Entre o valor da altura .......: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).unwrap();
    altura = a.trim().parse::<f32>().unwrap();

    volume = comprimento * largura * altura;

    println!("Volume ........................: {}", volume);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
