use std::io;
use std::io::prelude::*;

fn main() {

    let tupla = ("Laranja", 1000, 23.98);
    
    println!("1o. elemento da tupla {}", tupla.0);		
    println!("2o. elemento da tupla {}", tupla.1);		
    println!("3o. elemento da tupla {}", tupla.2);		

    println!();
    println!("Todos elementos da tupla {:?}", tupla);		

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
