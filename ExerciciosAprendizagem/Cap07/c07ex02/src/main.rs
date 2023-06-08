use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [i32; 5] = [0; 5];                 
    let mut s: i32 = 0;                
    
    let mut valor = String::new();   
    
    println!("Somatório de elementos impares\n");        
    
    // Entrada de dados
    
    for i in 0..5 {
        print!("Entre o {:2}o. valor: ", i + 1);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut valor).unwrap();
        a[i] = valor.trim().parse::<i32>().unwrap();
        valor.clear();
    }
    println!();

    // Processamento par ou impar

    for i in 0..5 {
        if a[i] % 2 != 0 {
            s += a[i];
	  }
    }
 
    // Apresentacao das matrizes

    println!("Soma elementos impares = {}.", s);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
