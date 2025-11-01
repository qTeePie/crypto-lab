mod number_theory;
mod sub_cipher;
use std::io::{self, Write};

fn main() {
    loop {
        clear_screen(); // ✨ wipe old output

        println!("✨🔐 Welcome to tiny Crypto Lab 🔐✨");
        println!("===============================================");
        println!("🧠 Menu for Cryptographic Functions:");
        println!("-----------------------------------------------");
        println!("1️. Number Theory");
        println!("2️⃣. Substitution Ciphers");
        println!("===============================================");

        print!("👉 Enter your choice (or type 'exit'): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if choice.eq_ignore_ascii_case("exit") {
            println!("💅 Bye.");
            break;
        }

        match choice.trim() {
            "1" => run_menu_section("Utils", || number_theory::menu::run()),
            _ => {
                println!("❌ Invalid choice, bestie. Try again 💅");
                pause("⏸ Press ENTER to return to menu...");
            }
            "2" => run_menu_section("Substitution Ciphers", || sub_cipher::run()),
        }
    }
}

/// Runs a section: clears screen, prints a header, runs the code, pauses after.
fn run_menu_section(section: &str, action: fn()) {
    clear_screen();
    println!("🚀 Running {section}...");
    action();
    pause("⏸ Press ENTER to return to menu...");
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
