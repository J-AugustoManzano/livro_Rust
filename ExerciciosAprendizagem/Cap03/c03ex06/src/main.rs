use std::io;
use std::io::prelude::*;

fn main() {    

    let mut lado_a = String::new();
    let mut lado_b = String::new();
    let mut lado_c = String::new();

    let a: f32;
    let b: f32;
    let c: f32;

    print!("Entre o lado <A>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_a).unwrap();
    a = lado_a.trim().parse::<f32>().unwrap();

    print!("Entre o lado <B>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_b).unwrap();
    b = lado_b.trim().parse::<f32>().unwrap();

    print!("Entre o lado <C>: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lado_c).unwrap();
    c = lado_c.trim().parse::<f32>().unwrap();

    println!();
    if a<b+c && b<a+c && c<a+b {
        if a==b && b==c {
            println!("Triângulo equilátero.");
        } else {
          if a==b || a==c || c==b {
              println!("Triângulo isósceles.");
          } else {
              println!("Triângulo escaleno.");
          }
        }
    } else {
        println!("Os valores fornecidos não formam um triângulo.");
    }

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
