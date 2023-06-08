use std::io;
use std::io::prelude::*;

fn proporcao() -> (f32, f32) {
    return (3.1415926535, 1.6180339887)
}

fn main() {
	
    let valor = proporcao();
    
    println!("Valor do pi ...: {:5.2}", valor.0);		
    println!("Valor do phi ..: {:5.2}", valor.1);	
    	
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
