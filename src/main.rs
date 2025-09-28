mod sub_cipher;
mod utils;
use std::io::{self, Write};

fn main() {
    loop {
        clear_screen(); // ✨ wipe old output

        println!("✨🔐 Welcome to tiny Crypto Lab 🔐✨");
        println!("===============================================");
        println!("🧠 Menu for Cryptographic Functions:");
        println!("-----------------------------------------------");
        println!("1️. Substitution Ciphers");
        println!("2️⃣. Compute GCD");
        println!("0️⃣. Exit");
        println!("===============================================");

        print!("👉 Enter your choice (or type 'exit'): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.eq_ignore_ascii_case("exit") || choice.trim() == "0" {
            println!("💅 Bye.");
            break;
        }

        match choice.trim() {
            "1" => {
                sub_cipher::run();
                pause("⏸ Press ENTER to return to menu...");
            }
            "2" => {
                let a_str = ask("🔢 Input number a: ");
                let b_str = ask("🔢 Input number b: ");

                let a: i64 = a_str.parse().unwrap();
                let b: i64 = b_str.parse().unwrap();

                let (g, u, v) = utils::gcd::extended_gcd(a, b);
                println!("gcd = {}, u = {}, v = {}", g, u, v);

                pause("⏸ Press ENTER to return to menu...");
            }
            _ => {
                println!("❌ Invalid choice, bestie. Try again 💅");
                pause("⏸ Press ENTER to return to menu...");
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

fn pause(message: &str) {
    println!("{message}");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}

fn clear_screen() {
    // cross-platform ANSI clear
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
