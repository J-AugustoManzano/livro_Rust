use std::io;
use std::io::prelude::*;
use std::mem::uninitialized;
use std::mem::forget;
use std::ptr;

fn troca<T>(a: &mut T, b: &mut T) {
    unsafe {
        let mut x: T = uninitialized();
        ptr::copy_nonoverlapping(&*a, &mut x, 1);
        ptr::copy_nonoverlapping(&*b, a, 1);
        ptr::copy_nonoverlapping(&x, b, 1);
        forget(x);
    }
}

fn main() {
    
    let mut valor_a = 1;
    let mut valor_b = 2;
    
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    troca(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    let mut valor_a = 3.5;
    let mut valor_b = 4.5;
    
    println!("\n");
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    troca(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    let mut valor_a = 'a';
    let mut valor_b = 'b';
    
    println!("\n");
    println!("Valor A antes da troca ...: {}", valor_a);
    println!("Valor B antes da troca ...: {}", valor_b);
    
    troca(&mut valor_a, &mut valor_b);

    println!();
    println!("Valor A depois da troca ..: {}", valor_a);
    println!("Valor B depois da troca ..: {}", valor_b);
    
    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();

}
