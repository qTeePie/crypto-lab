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
        // r0, r1 = current remainders
        // x0, x1 = coefficients for `a` that correspond to r0, r1
        while r1 != 0 {
            let q = r0 / r1;

            // 1) Compute the *next* pair
            let r_next = r0 - q * r1; // this is r_{k+1}
            let x_next = x0 - q * x1; // this is x_{k+1} (coeff for r_{k+1})

            // 2) Shift the "current" pair forward together
            r0 = r1; // (r_{k-1}, r_k) -> (r_k, ...)
            x0 = x1; // keep r0 aligned with its coeff x0

            // 3) Install the *next* pair together
            r1 = r_next; // ... -> (..., r_{k+1})
            x1 = x_next; // align r1 with x1
        }
    }

    let g = r0;
    let x = x0;
    let y = (g - a * x) / b;

    // g is the GCD, x = u and y = v
    (g, x, y)
}
