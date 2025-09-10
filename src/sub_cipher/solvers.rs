pub fn caesar_cipher(input: &str, shift: i8) -> String {
    input.chars().map(|c| shift_char(c, shift)).collect()
}

pub fn shift_char(c: char, shift: i8) -> char {
    let a = if c.is_ascii_uppercase() {
        b'A'
    } else if c.is_ascii_lowercase() {
        b'a'
    } else {
        return c;
    };

    let shifted = ((c as u8 - a + (shift as u8 % 26) + 26) % 26) + a;
    shifted as char
}
