use crate::sub_cipher::solvers;
use std::io::{self, Write};

pub fn run() {
    loop {
        println!("\n=== Substitution Ciphers ===");
        let msg = ask("🔤 Enter your message (or type 'exit'): ");
        if msg.eq_ignore_ascii_case("exit") {
            println!("💅 Bye.");
            break;
        }

        println!("1) Encrypt with Caesar 🔒");
        println!("2) Decrypt with Caesar 🔑");

        let choice = ask("👉 Choice: ");

        match choice.trim() {
            "1" => {
                let shift: i8 = ask("🔢 Shift: ").parse().unwrap_or(0);
                println!("shift {}", shift);
                println!("Encrypted: {}", solvers::caesar_cipher(&msg, shift));
            }
            "2" => {
                let shift: i8 = ask("🔢 Shift: ").parse().unwrap_or(0);
                println!("Decrypted: {}", solvers::caesar_cipher(&msg, -shift));
            }
            _ => println!("❌ Invalid choice"),
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
