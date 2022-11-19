use std::io;


// pegando a e-nésima sequência de Fibonacci 
// fórmula:

//  F(1) + F(2) + F(3) + ... + F(n) = F(n + 2) − 1 para n ≥ 1

fn main() {
    println!("\t<a sequência de fibonacci>");
    println!(":ajuda: aperte 'q' para sair\n");
    
    loop {
        let mut n = String::new();
        
        println!("insira um número inteiro.");
        
        io::stdin()
            .read_line(&mut n)
            .expect("falhou ao ler a linha");
            
        if n.trim() == "q" {
            println!("\ntchau!!");
            break;
        }
        
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("o n-ésimo número de fibonacci é: {}", fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
