use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [[i64; 5]; 5] = [[0; 5]; 5];                 
    let mut b: [[i64; 5]; 5] = [[0; 5]; 5];                 
    
    let mut valor = String::new();   
    
    for i in 0 .. 5 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 5 {
            print!("Entre o valor da coluna ..: {}: ", j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            a[j][i] = valor.trim().parse::<i64>().unwrap();
            valor.clear();
        }
        println!();
    }
    
    for i in 0 .. 5 {
        for j in 0 .. 5 {
            if i + j == 4 {
                b[j][i] = a[j][i] * 3;
            } else {
                b[j][i] = a[j][i] * 2;
		    }
        }
    }
    
    for i in 0 .. 5 {
        println!("Linha ...: {}", i + 1);
        for j in 0 .. 5 {
            println!("Coluna ..: {} = {}", j + 1, b[j][i]);
        }
        println!();
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

