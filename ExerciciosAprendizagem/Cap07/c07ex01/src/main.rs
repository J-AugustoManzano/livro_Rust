use std::io;
use std::io::prelude::*;

fn main() {

    let mut a = [0; 10];                
    let mut b = [0; 10];
    
    let mut valor = String::new();   
    
    println!("Exemplo de checagem de índice\n");        
    
    // Entrada de dados
    
    for i in 0..10 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }
    println!();

    // Processamento par ou impar

    for i in 0..10 {
        if i % 2 == 0 {
          b[i] = a[i] * 5;
        } else {
          b[i] = a[i] + 5;
	    }
    }

    // Apresentacao das matrizes

    for i in 0..10 {
        println!("A[{:2}] = {:4} na posição {}.", i + 1, a[i], i);
    }
    println!();

    for i in 0..10 {
        println!("B[{:2}] = {:4} na posição {}.", i + 1, b[i], i);
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
