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

(read page 14 claims bottom page for explanation for O(log n))

```rust
fn gcd(a: i32, b: i32) -> i32 {
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

### GCD Extended

```rust
pub fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut r0 = a;
    let mut r1 = b;

    let mut x0 = 1; // coeff for a
    let mut x1 = 0;

    while r1 != 0 {
        let q = r0 / r1;

        let new_r = r0 - q * r1; // * see comment
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
```

> **\*** confused me so much, the below explanation from chattie helped me see clearly. Had to save.

Let’s re-do the **backward substitution** but only pay attention to the $a$-part each time. ignoring $v$ completely and just follow $u$.

---

## Step 1: First division

$$
101 = 4\cdot 23 + 9
$$

Rearrange:

$$
9 = 101 - 4\cdot 23
$$

Look only at the $a$-coefficient:

$$
u = 1
$$

So remainder 9 has $u=1$.

---

## Step 2: Second division

$$
23 = 2\cdot 9 + 5
$$

$$
5 = 23 - 2\cdot 9
$$

Now substitute the $a$-part of 9 (from Step 1: $u=1$):

$$
5 = (…)\; - 2(1\cdot 101 + …)
$$

So the $a$-coefficient is:

$$
u = -2
$$

---

## Step 3: Third division

$$
9 = 1\cdot 5 + 4
$$

$$
4 = 9 - 5
$$

Substitute the $a$-parts we know:

- For 9: $u=1$
- For 5: $u=-2$

So:

$$
u = 1 - (-2) = 3
$$

---

## Step 4: Fourth division

$$
5 = 1\cdot 4 + 1
$$

$$
1 = 5 - 4
$$

Substitute the $a$-parts:

- For 5: $u=-2$
- For 4: $u=3$

So:

$$
u = -2 - 3 = -5
$$

---

## Final result

$$
\gcd(101, 23) = 1 = (-5)\cdot 101 + (…)\cdot 23
$$

We don’t know $v$ yet — but we don’t care.
The important part: $u = -5$.
