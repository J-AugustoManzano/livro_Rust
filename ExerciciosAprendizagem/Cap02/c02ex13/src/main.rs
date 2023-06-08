use std::io;
use std::io::prelude::*;

fn main() {    

    println!("abs(-3.5) ........: {}", -3.5f64.abs());
    println!("2 ^ 3 ............: {}", 2i32.pow(3));
    println!("sqrt 25 ..........: {}", 25f64.sqrt());
    println!("cbrt 27 ..........: {}", 27f64.cbrt());
    println!("round 2.45 .......: {}", 2.45f64.round());
    println!("floor 2.10 .......: {}", 2.10f64.floor());
    println!("ceiling 2.10 .....: {}", 2.10f64.ceil());
    println!("e ^ 2 ............: {}", 2f64.exp());
    println!("log(2) ...........: {}", 2f64.ln());
    println!("log10(2) .........: {}", 2f64.log10());
    println!("90 to Radians ....: {}", 90f64.to_radians());
    println!("PI to Degrees ....: {}", 3.14f64.to_degrees());
    println!("max 4, 5 .........: {}", 4f64.max(5f64));
    println!("min 4, 5 .........: {}", 4f64.min(5f64));
    println!("sin 3.14 .........: {}", 3.14f64.sin());
    println!("cos 3.14 .........: {}", 3.14f64.cos());
    println!("tan 3.14 .........: {}", 3.14f64.tan());
    println!("asin 3.14 ........: {}", 3.14f64.asin());
    println!("acos 3.14 ........: {}", 3.14f64.acos());
    println!("atan 3.14 ........: {}", 3.14f64.atan());
    println!("sinh 3.14 ........: {}", 3.14f64.sinh());
    println!("cosh 3.14 ........: {}", 3.14f64.cosh());
    println!("tanh 3.14 ........: {}", 3.14f64.tanh());

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
