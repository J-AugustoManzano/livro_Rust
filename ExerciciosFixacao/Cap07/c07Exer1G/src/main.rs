use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [i64;  6] = [0;  6];              
    let mut b: [i64;  6] = [0;  6];
    let mut c: [i64; 12] = [0; 12];
    
    let mut valor = String::new();
    let mut valido: i64;   
    
    for i in 0 .. 6 {
		loop {
            print!("Entre o {:2}o. valor: ", i + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            valido = valor.trim().parse::<i64>().unwrap();
            valor.clear();
            if valido % 2 == 0 { a[i] = valido; break; }   
        }        
    }
    println!();
    
    for i in 0 .. 6 {
		loop {
            print!("Entre o {:2}o. valor: ", i + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            valido = valor.trim().parse::<i64>().unwrap();
            valor.clear();
            if valido % 2 != 0 { b[i] = valido; break; }
        }        
    }
    println!();

    for i in 0 .. 12 {
       if i <= 5 {
           c[i] = a[i];
       } else {
           c[i] = b[i - 6];   
       }
    }

    for i in 0 .. 12 {
        println!("C[{:2}] = {}", i + 1, c[i]);
    }


    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
