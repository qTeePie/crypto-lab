use crate::number_theory::factors::gcd::extended_gcd;

pub fn inverse_by_extended_euclidean(a: i64, p: i64) -> Option<i64> {
    let (g, u, _) = extended_gcd(a, p);

    if g != 1 {
        return None; // inverse does NOT exist
    }

    // Normalize u into the modular range
    let inv = (u % p + p) % p;
    Some(inv)
}
