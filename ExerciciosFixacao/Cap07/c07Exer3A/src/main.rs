use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [[i32; 5]; 3] = [[0; 5]; 3];                 
    let mut b: [[i32; 5]; 3] = [[0; 5]; 3];                 
    let mut c: [[i32; 5]; 3] = [[0; 5]; 3];                 
    
    let mut valor = String::new();   
    
    for i in 0 .. 5 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 3 {
            print!("Entre o valor da coluna ..: {}: ", j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            a[j][i] = valor.trim().parse::<i32>().unwrap();
            valor.clear();
        }
        println!();
    }
    println!();
    
    for i in 0 .. 5 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 3 {
            print!("Entre o valor da coluna ..: {}: ", j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            b[j][i] = valor.trim().parse::<i32>().unwrap();
            valor.clear();
        }
        println!();
    }

    for i in 0 .. 5 {
        for j in 0 .. 3 {
            c[j][i] = a[j][i] + b[j][i];
        }
    }
    
    for i in 0 .. 5 {
        println!("Linha ...: {}", i + 1);
        for j in 0 .. 3 {
            println!("Coluna ..: {} = {}", j + 1, c[j][i]);
        }
        println!();
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

