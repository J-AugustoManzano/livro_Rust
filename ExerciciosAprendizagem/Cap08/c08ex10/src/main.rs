use std::io;
use std::io::prelude::*;
use std::f64::consts::PI;

enum FiguraGeometrica {
    Cilindro(f64, f64),
    Quadrado(f64),
    Trapezio(f64, f64, f64),
}

fn main() {
	
    let area_cil = FiguraGeometrica::Cilindro(15.0, 7.0);
    let area_qua = FiguraGeometrica::Quadrado(2.5);
    let area_tra = FiguraGeometrica::Trapezio(1.5, 2.5, 10.0);

    if let FiguraGeometrica::Cilindro(raio, altura) = area_cil {
        let area: f64;
        area = 2.0 * PI * raio * (raio + altura);
        println!("Área da circunferência ..: {:8.2}", area);
    }

    if let FiguraGeometrica::Quadrado(lado) = area_qua {
        println!("Área do quadrado ........: {:8.2}", lado.powf(2.0));
    }

    if let FiguraGeometrica::Trapezio(bs_maior, bs_menor, altura) = area_tra {
        let area: f64;
        area = ((bs_maior + bs_menor) * altura) / 2.0;
        println!("Área do trapézio ........: {:8.2}", area);
    }
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
