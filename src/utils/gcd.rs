/// Returns the GCD of the two input numbers `a` and `b`.
pub fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;

    if x == y {
        return x;
    }

    if y > x {
        std::mem::swap(&mut x, &mut y);
    }

    while y > 0 {
        let temp = x;
        x = y;
        y = temp % y;
    }

    return x;
}

/// The extended GCD algorithm computes integers u and v such that au + bv = GCD
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r0 = a;
    let mut r1 = b;

    let mut x0 = 1; // coeff for a
    let mut x1 = 0;

    while r1 != 0 {
        let q = r0 / r1;

        let new_r = r0 - q * r1;
        r0 = r1;
        r1 = new_r;

        let new_x = x0 - q * x1;
        x0 = x1;
        x1 = new_x;
    }

    let g = r0;
    let x = x0;
    let y = (g - a * x) / b;

    // g is the GCD, x = u and y = v
    (g, x, y)
}
