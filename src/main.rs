mod sub_cipher;
mod utils;
use std::io::{self, Write};

fn main() {
    println!("✨🔐 Welcome to tiny Crypto Lab 🔐✨");
    println!("===============================================");
    println!("🧠 Menu for Cryptographic Functions:");
    println!("-----------------------------------------------");
    println!("1️. Substitution Ciphers");
    println!("0️⃣. Exit");
    println!("===============================================");

    loop {
        print!("👉 Enter your choice (0-2): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                sub_cipher::run();
            }
            "2" => {
                let a_str = ask("🔢 Input number a: ");
                let b_str = ask("🔢 Input number b: ");

                let a: i64 = a_str.parse().unwrap();
                let b: i64 = b_str.parse().unwrap();

                let (g, u, v) = utils::gcd::extended_gcd(a, b);
                println!("gcd = {}, u = {}, v = {}", g, u, v);
            }
            "-1" => {
                println!("💅 Bye.");
                break;
            }
            _ => {
                println!("❌ Invalid choice, bestie. Try again 💅");
            }
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
