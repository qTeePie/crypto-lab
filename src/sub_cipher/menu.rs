use super::solvers;
use std::io::{self, Write};

pub fn run() {
    println!("Substitution Ciphers Submenu");
    println!("===========================");

    print!("🔤 Enter your message: ");
    io::stdout().flush().unwrap(); //
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    print!("🔢 Enter shift amount (e.g. 3): ");
    io::stdout().flush().unwrap();
    let mut shift_str = String::new();
    io::stdin().read_line(&mut shift_str).unwrap();

    let shift: i8 = shift_str.trim().parse().unwrap_or(0);

    let encrypted = solvers::caesar_cipher(&message.trim(), shift);
    println!("🔒 Encrypted Message: {encrypted}");
}
