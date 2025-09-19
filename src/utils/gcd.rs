fn gcd_euclidean(a: usize, b: usize) -> usize {
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
