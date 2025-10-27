use crate::number_theory::factors::{gcd, ord_p};
use std::io::{self, Write};

pub fn run() {
    loop {
        println!("\n=== 🧮 Utils Playground ===");
        println!("1) Extended gcd(a, b)");
        println!("2) Compute ord_p(n, p)");

        match ask("👉 Enter your choice (or type 'exit')").trim() {
            "1" => {
                let a: i64 = ask("a = ").parse().unwrap_or(0);
                let b: i64 = ask("b = ").parse().unwrap_or(0);
                let (g, u, v) = gcd::extended_gcd(a, b);
                println!("gcd = {}, u = {}, v = {}", g, u, v);
            }
            "2" => {
                let n: u64 = ask("n = ").parse().unwrap_or(0);
                let p: u64 = ask("p = ").parse().unwrap_or(0);
                println!("ord_{}({}) = {}", p, n, ord_p::ord_p(n, p));
            }
            "exit" => {
                println!("💅 bye.");
                break;
            }
            _ => println!("❌ invalid choice"),
        }
    }
}

fn ask(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
