use std::io;
use std::io::prelude::*;

fn main() {

    let mut nota: [[f32; 10]; 4] = [[0.; 10]; 4];                 
    
    let mut valor = String::new();   
    
    println!("Leitura e apresentação de notas escolares.\n");        

    println!("Entrada dos dados.\n");        
    
    // Entrada das notas
    
    for i in 0..10 {
        println!("Informa as notas do {:2}o. aluno:\n", i + 1);
        for j in 0..4 {
            print!("Nota {}: ", j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            nota[j][i] = valor.trim().parse::<f32>().unwrap();
            valor.clear();
        }
        println!();
    }

    println!("Saída dos dados.\n");        

    // Apresentação das notas

    for i in 0..10 {
        println!("As notas do {:2}o. aluno são:\n", i + 1);
        for j in 0..4 {
            println!("Nota {} = {:6.2}", j + 1, nota[j][i]);
        }
        println!();
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
