/// Efficient approach to computing factors of a number.
/// Time Complexity: O( sqrt(n) )
fn list_factors(number: i64) -> Vec<i64> {
    // Initialize factors Vector
    let mut factors: Vec<i64> = Vec::new();

    let mut i: i64 = 1;

    // Check till i is less than or equal to square root n
    while i * i <= number {
        if number % i == 0 {
            factors.push(i);

            // to prevent duplication, if number is perfect square
            if i * i != number {
                factors.push(number / i);
            }
        }
        i += 1;
    }

    // It is generally useful to sort the vector
    // And it will not affect our time complexity,
    // Because logarithmic time is much less than square root time complexity

    factors.sort();

    // Return the factors Vector as answer
    return factors;
}

/// Naive approach to computing factors of a number.
/// Time Complexity: O(n)
fn list_factors_naive(number: i64) -> Vec<i64> {
    // Initialize factors Vector
    let mut factors: Vec<i64> = Vec::new();

    // Check all the numbers from 1 to n, both inclusive
    for i in 1..(number + 1) {
        if number % i == 0 {
            // Push the number to factors, if it divides number
            factors.push(i);
        }
    }

    // Return the factors Vector as answer
    return factors;
}
