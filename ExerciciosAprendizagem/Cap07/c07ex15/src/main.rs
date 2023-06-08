use std::io;
use std::io::prelude::*;
 
fn main() {
	
    let valores: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    
    for i in 0 .. valores.len() {
        println!("{}", valores[i]);	
    }

    println!();
       
    for i in &valores {
        println!("{}", i);	
    }
       
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
