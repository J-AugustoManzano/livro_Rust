use std::io;
use std::io::prelude::*;
use std::cmp::PartialOrd;

fn menorvlr<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {return a;} else {return b;}
}

fn main() {
	
    let v1: u16 = 5;
    let v2: u16 = 4;
	
    let v3: u64 = 2;
    let v4: u64 = 9;
	
    let v5: f64 = 2.9;
    let v6: f64 = 5.8;

    let v7: f64 = 9.9;
    let v8: f64 = 6.7;
	
    println!("Entre {} e {} o menor valor é {}", v1, v2, menorvlr(&v1, &v2));
    println!("Entre {} e {} o menor valor é {}", v3, v4, menorvlr(&v3, &v4));
    println!("Entre {} e {} o menor valor é {}", v5, v6, menorvlr(&v5, &v6));
    println!("Entre {} e {} o menor valor é {}", v7, v8, menorvlr(&v7, &v8));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
