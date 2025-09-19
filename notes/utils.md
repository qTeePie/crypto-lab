# Notes

A nots sheet for specs on the utils in this solution, with emphasis on time complexity.

| Common Big O | What It Means | Examples                          |
| ------------ | ------------- | --------------------------------- |
| O(1)         | Constant time | Access array element, hash lookup |
| O(log n)     | Logarithmic   | Binary search, GCD                |
| O(n)         | Linear        | Loop through array                |
| O(n log n)   | Fast sorting  | Merge sort, quicksort             |
| O(n²)        | Nested loops  | Brute-force comparisons           |

---

## GCD - Greatest common divisor

### Euclidean

```rust
fn gcd(a: usize, b: usize) -> usize {
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
```

### Step 1. Start with the definition

By definition:

$$
\log_2 n = k \quad \text{iff} \quad 2^k = n.
$$

That’s the only true definition. Keep that in mind.

---

### Step 2. Rearrange it

If $2^k = n$, then dividing both sides by $2^k$:

$$
n / 2^k = 1.
$$

---

### Step 3. Interpret it

That equation says:

> “If you divide $n$ by 2 exactly $k$ times, you will reach 1.”

Because dividing by 2 once is the same as multiplying by $1/2$. Doing it $k$ times is the same as multiplying by $1/2^k$.

It’s the number $k$ of halvings you need until you reach 1.

---
