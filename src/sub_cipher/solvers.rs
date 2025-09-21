pub fn caesar_cipher(input: &str, shift: i8) -> String {
    // send each character to shift_char with the chosen shift
    // => collect result to string and return
    input.chars().map(|c| shift_char(c, shift)).collect()
}

pub fn shift_char(c: char, shift: i8) -> char {
    // only working with ascii 💅
    if !c.is_ascii_alphabetic() {
        return c; // // keeps any emojis etc as is
    }

    // set starting point, 'a' for lowercase 'A' uppercase
    let base = if c.is_ascii_uppercase() {
        b'A' as i8
    } else {
        b'a' as i8
    };

    let c_pos = c as i8 - base; // position of character in alphabet, subtract base from ASCII value of c
    let shifted = (c_pos + shift).rem_euclid(26);
    (base + shifted) as u8 as char // return the character by adding base

    /*
        How does .rem_euclid(m) work?

        eg. shift= -3

        (-3 % 26 + 26) % 26
        = (-3 + 26) % 26
        = 23 % 26
        = 23 ✅

        Connected to euclidean method, defining division as:
        a = b * q + r
        where 0 ≤ r < |b|
        => forcing positive remainder
    */
}
