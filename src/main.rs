mod sub_cipher;
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
        print!("👉 Enter your choice (0-1): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                sub_cipher::run();
            }
            "2" => {
                println!("💅 Bye.");
                break;
            }
            _ => {
                println!("❌ Invalid choice, bestie. Try again 💅");
            }
        }
    }
}
