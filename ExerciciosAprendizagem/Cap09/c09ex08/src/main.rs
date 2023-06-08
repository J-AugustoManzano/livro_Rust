use std::io;
use std::io::prelude::*;

fn f(x: i32) -> i32 {
    return x + 3;
}

fn g(x: i32) -> i32 {
    return x * 2;
}

fn h(x: i32) -> i32 {
    return f(g(x));
}

fn main() {
	
    println!("{}", h(5));
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
