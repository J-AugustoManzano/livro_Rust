use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [f32; 5] = [0.; 5]; // ou: let mut a = [0.; 5];                 
    let mut s: f32 = 0.;                
    
    let mut valor = String::new();   
    
    println!("Somatório de elementos reais\n");        
    
    // Entrada de dados
    
    for i in 0..5 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f32>().unwrap();
        valor.clear();
    }
    println!();

    // Somatório

    for i in 0..5 {
        s += a[i];
    }

    // Apresentacao das matrizes

    println!("Soma = {}.", s);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
