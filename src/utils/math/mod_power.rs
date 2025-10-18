/*
  result · b^e ≡ base^exp (mod modulus)
*/

/// Computes (base^exp) mod modulus using fast exponentiation.
/// All parameters are i64 for safety and consistency.
///
/// - `modulus`: the modulo value
/// - `base`: the base number
/// - `exp`: the exponent
pub fn mod_pow(modulus: i64, base: i64, exp: i64) -> i64 {
    if modulus == 0 {
        return -1; // avoid division by zero
    }

    let mut result: i64 = 1; // keeps track of result
    let mut b = mod_wrap(base, modulus); // current power
    let mut e = exp; // exponent

    // process the exponents bit one by one
    while e > 0 {
        if e % 2 == 1 {
            // odd number means bit is 1 -> append to result
            result = result * b % modulus;
        }
        // square b and reduce e to shift right by one bit
        b = (b * b) % modulus;
        e /= 2;
    }

    result
}

/// Wraps negative remainders into [0, modulus) range.
fn mod_wrap(x: i64, m: i64) -> i64 {
    let r = x % m;
    if r < 0 { r + m } else { r }
}
