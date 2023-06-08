use std::io;
use std::io::prelude::*;

fn main() {

    let tamanho: usize;    
    let mut valor = String::new();   
    let mut quant = String::new();   
    
    println!("Matriz unidimensional dinâmica\n");        
    
    // Entrada dos elementos

    print!("Entre o tamanho ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut quant).unwrap();
    tamanho = quant.trim().parse::<usize>().unwrap();
    println!();

    let mut a = vec![0.; tamanho]; // matriz dinâmica   
    
    for i in 0..tamanho {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<f32>().unwrap();
        valor.clear();
    }
    println!();

    // Apresentação dos elementos

    for i in 0..tamanho {
        println!("A[{:2}] = {:8.2} na posição {}.", i + 1, a[i], i);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
