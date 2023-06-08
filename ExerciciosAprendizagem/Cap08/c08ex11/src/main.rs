use std::io;
use std::io::prelude::*;

struct Lado {
    base: f32,
    altura: f32,
}

struct Retangulo {
    lateral: Lado,
}

fn main() {
    
    let lado: Lado;
    let retangulo: Retangulo;
    let area: f32;
            
    lado = Lado {base: 4.789, altura: 2.975};

    let Lado {base: bas, altura: alt} = lado;

    retangulo = Retangulo {lateral: Lado {base: bas, altura: alt}};
    area = retangulo.lateral.altura * retangulo.lateral.base;

    println!("Retângulo");
    println!("");
    println!("Lado da base ...: {:8.2}", retangulo.lateral.base);
    println!("Lado da altura .: {:8.2}", retangulo.lateral.altura);
    println!("");
    println!("Área ...........: {:8.2}", area);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
