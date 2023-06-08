use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [[i32; 5]; 6] = [[0; 5]; 6];                 
    let mut b: [[i32; 5]; 6] = [[0; 5]; 6];                 
    let mut c: [[i32; 5]; 6] = [[0; 5]; 6];                 
    
    let mut valor = String::new(); 
    let mut valido: i32;     
    
    for i in 0 .. 5 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 6 {
			loop {
                print!("Entre o valor da coluna ..: {}: ", j + 1);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut valor).unwrap();
                valido = valor.trim().parse::<i32>().unwrap();
                valor.clear();
                if valido % 2 == 0 { a[j][i] = valido; break; }  
            }
        }
        println!();
    }
    println!();
    
    for i in 0 .. 5 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 6 {
			loop {
                print!("Entre o valor da coluna ..: {}: ", j + 1);
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut valor).unwrap();
                valido = valor.trim().parse::<i32>().unwrap();
                valor.clear();
                if valido % 2 != 0 { b[j][i] = valido; break; }  
            }
        }
        println!();
    }

    for i in 0 .. 5 {
        for j in 0 .. 6 {
            c[j][i] = a[j][i] + b[j][i];
        }
    }
    
    for i in 0 .. 5 {
        println!("Linha ...: {}", i + 1);
        for j in 0 .. 6 {
            println!("Coluna ..: {} = {}", j + 1, c[j][i]);
        }
        println!();
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}

