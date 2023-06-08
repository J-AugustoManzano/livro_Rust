use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Coordenada {
    x: u32,
    y: u32,
}

fn main() {

    let posicao = Coordenada { x: 5, y: 9 };
    let palavra = String::from("Teste");
    let numero1 = 26;
    let numero2 = 1.24;
    let numero3 = 1.984;
    let numero4 = 1e6;
    let numero5 = 255;

    println!("Posicao .....: {:#?}", posicao);
    println!("Palavra .....: {:>15}", palavra);
    println!("Palavra .....: {:<15}", palavra);
    println!("Número 1 ....: {:05}", numero1);
    println!("Número 2 ....: {:.3}", numero2);
    println!("Número 3 ....: {:.05}", numero3);
    println!("Número 4 e ..: {:e}", numero4);
    println!("Número 4 E ..: {:E}", numero4);
    println!("Número 5 h ..: {:x}", numero5);
    println!("Número 5 H ..: {:X}", numero5);
    println!("Número 5 o ..: {:o}", numero5);
    println!("Número 5 b ..: {:b}", numero5);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
