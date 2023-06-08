use std::io;
use std::io::prelude::*;

fn main() {

    let mut a: [[f32; 8]; 8] = [[0.0; 8]; 8];                 
    
    let mut valor = String::new();   
    
    let mut soma: f32 = 0.0;
    
    for i in 0 .. 8 {
        println!("Entre o valor da linha ...: {}", i + 1);
        for j in 0 .. 8 {
            print!("Entre o valor da coluna ..: {}: ", j + 1);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut valor).unwrap();
            a[j][i] = valor.trim().parse::<f32>().unwrap();
            valor.clear();
        }
        println!();
    }
    
    for i in 0 .. 8 {
        for j in 0 .. 8 {
            if i == j {
                soma += a[j][i];
		    }
        }
    }
    
    println!("Soma diagonal principal = {}", soma);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
