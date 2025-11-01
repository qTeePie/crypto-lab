use crate::number_theory::factors::gcd::extended_gcd;
use crate::number_theory::factors::is_prime::is_prime;
use crate::number_theory::factors::mod_power::mod_pow;

pub fn inverse(a: i64, m: i64) -> Option<i64> {
    if is_prime(m) {
        Some(mod_pow(m, a, m - 2))
    } else {
        inverse_by_extended_euclidean(a, m)
    }
}

pub fn inverse_by_extended_euclidean(a: i64, m: i64) -> Option<i64> {
    let (g, u, _) = extended_gcd(a, m);

    if g != 1 {
        return None; // inverse does NOT exist
    }

    // Normalize u into the modular range
    let inv = (u % m + m) % m;
    Some(inv)
}
