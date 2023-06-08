use std::io;
use std::io::prelude::*;
 
fn main() {
	
    let valores: [i32; 5] = [1, 2, 3, 4, 5];

    let total: i32 = valores.iter().sum();
    
    println!("{}", total);	
       
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
