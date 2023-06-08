use std::io;
use std::io::prelude::*;

trait CalcArea {
    fn area(&self) -> f32;
}

struct Circunferencia {
    raio: f32,
    } impl CalcArea for Circunferencia {
    fn area(&self) -> f32 {
        return self.raio.powi(2) * std::f32::consts::PI
    }
}

struct Quadrado {
    lado: f32,
    } impl CalcArea for Quadrado {
    fn area(&self) -> f32 {
        return self.lado.powi(2)
    }
}

fn calcarea(figura: &CalcArea) {
    println!("Área calculada = {}", figura.area())
}

fn main() {

    let mut entra_lado = String::new();
    let mut entra_raio = String::new();

    let raio_ent: f32;
    let lado_ent: f32;
	
    print!("Entre o valor do lado de um quadrado: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entra_lado).unwrap();
    lado_ent = entra_lado.trim().parse::<f32>().unwrap();
    calcarea(&Quadrado{lado: lado_ent});
	
    println!();
    print!("Entre o valor do raio de uma circunferência: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entra_raio).unwrap();
    raio_ent = entra_raio.trim().parse::<f32>().unwrap();
    calcarea(&Circunferencia{raio: raio_ent});
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}


