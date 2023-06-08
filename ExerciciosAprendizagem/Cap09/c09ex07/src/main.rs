use std::io;
use std::io::prelude::*;

fn fatorialcomum(x: i8) -> i64 {
    let mut f: i64 = 1;
    if x == 0 || x == 1 {
        return 1;
    }
    if x < 0 || x > 20 {
        return 0;
    } else {
        for i in 1 .. x + 1 {
            f *= i as i64;
        }
        return f;
    }
}

fn fatorialpura(x: i8) -> i64 {
    if x == 0 || x == 1 {
        return 1;
    }
    if x < 0 || x > 20 {
        return 0;
    } else {
        return fatorialpura(x - 1) * x as i64;
    }
}

fn main() {

    // Uso da sub-rotina de função comum

    println!("Resultados da função comum\n");
    println!("-1! = {}", fatorialcomum(-1));
    println!(" 0! = {}", fatorialcomum(0));
    println!(" 1! = {}", fatorialcomum(1));
    println!(" 5! = {}", fatorialcomum(5));
    println!("20! = {}", fatorialcomum(20));
    println!("21! = {}", fatorialcomum(21));
    println!();

    // Uso da sub-rotina de função pura

    println!("Resultados da função pura\n");
    println!("-1! = {}", fatorialpura(-1));
    println!(" 0! = {}", fatorialpura(0));
    println!(" 1! = {}", fatorialpura(1));
    println!(" 5! = {}", fatorialpura(5));
    println!("20! = {}", fatorialpura(20));
    println!("21! = {}", fatorialpura(21));

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
