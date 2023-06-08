use std::io;
use std::io::prelude::*;
use std::cmp::PartialOrd;

fn maiorvlr<T: PartialOrd + Copy>(vetor: &[T]) -> T {
	
    let mut maior = vetor[0];

    for &atual in vetor.iter() {
        if atual > maior {
            maior = atual;
        }
    }

    return maior;
    
}

fn main() {
	
    let numeros = vec![20, 25, 15, 98, 33, 78, 97, 10, 77, 45];
    let letras = vec!['F', 'A', 'X', 'T', 'W', 'C', 'G', 'H', 'U', 'B'];

    println!("Maior valor numérico ...: {}", maiorvlr(&numeros));
    println!("Maior letra ............: {}", maiorvlr(&letras));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

