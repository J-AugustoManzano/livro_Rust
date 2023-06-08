use std::io;
use std::io::prelude::*;

fn main() {

    let linhas: usize;   
    let colunas: usize;   

    let mut valor = String::new();   
    let mut lin = String::new();   
    let mut col = String::new();   
    
    println!("Matriz unidimensional dinâmica\n");        
    
    // Entrada dos elementos

    print!("Entre a quantidade de linhas ...: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lin).unwrap();
    linhas = lin.trim().parse::<usize>().unwrap();

    print!("Entre a quantidade de colunas ..: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut col).unwrap();
    colunas = col.trim().parse::<usize>().unwrap();
    println!();

    let mut a = vec![vec![0; linhas]; colunas]; // matriz dinâmica   
    
    for i in 0..linhas {
        for j in 0..colunas {
            print!("Informe posição [{},{}]: ", j + 1, i + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            a[j][i] = valor.trim().parse::<i32>().unwrap();
            valor.clear();
        }
        println!();
    }

    // Apresentação das notas

    println!();        
    for i in 0..linhas {
        for j in 0..colunas {
            println!("Posição [{},{}] = {}", j + 1, i + 1, a[j][i]);
        }
        println!();
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
