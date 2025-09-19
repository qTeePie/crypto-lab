## 🧠 Big O Cheatsheet: “How Screwed Am I?”

| Big O          | Nickname                     | What It Means                            | Example Patterns                           | Vibe Check                                 |
| -------------- | ---------------------------- | ---------------------------------------- | ------------------------------------------ | ------------------------------------------ |
| **O(1)**       | ✨ _Instant Slay_ ✨         | Doesn’t grow with input size             | Hash lookup, `arr[3]`, `return x + 1`      | 💅 chill queen                             |
| **O(log n)**   | 📉 _Divide & Conquer Baddie_ | Grows slowly — each step cuts input down | Binary search, Euclidean GCD               | 🐍 sneaky efficient                        |
| **O(n)**       | 🏃 _Reasonable Hustler_      | One step per item                        | Simple loops, array traversal              | 🧃 tolerable                               |
| **O(n log n)** | 🪄 _Efficient Sorcerer_      | Log work per item                        | Merge sort, Quick sort (average)           | 🧠 high-tier clean girl                    |
| **O(n²)**      | 😫 _Nested Hell_             | For each item, touch all others          | Double loops, brute-force comparisons      | ☠ avoid unless you're suffering on purpose |
| **O(2ⁿ)**      | 😵 _Exponential Doom_        | Work doubles each time input grows       | Recursive subset generation, bad recursion | 🔥 burn it with fire                       |
| **O(n!)**      | 🚨 _Permutation Panic_       | Try every possible ordering              | Brute-force traveling salesman             | 🚫 math war crime                          |

---

## 📚 Common Scenarios

| Situation                          | Complexity       | Notes                    |
| ---------------------------------- | ---------------- | ------------------------ |
| Loop over array once               | `O(n)`           | Classic for loop         |
| Nested loops                       | `O(n²)`          | Brutal if `n` is large   |
| Binary search                      | `O(log n)`       | Splits input each step   |
| Merge sort / quicksort             | `O(n log n)`     | Efficient for large data |
| HashMap lookup                     | `O(1)` (average) | Unless collision hell    |
| GCD (Euclidean Algorithm)          | `O(log n)`       | Smart math boi           |
| Recursion with 2 branches per call | `O(2ⁿ)`          | FLEE unless memoized     |

---

## 🪜 How They Grow (When n = 10, 100, 1000)

| `n`  | O(1) | O(log n) | O(n) | O(n log n) | O(n²)  | O(2ⁿ) |
| ---- | ---- | -------- | ---- | ---------- | ------ | ----- |
| 10   | 1    | \~3      | 10   | \~30       | 100    | 1024  |
| 100  | 1    | \~7      | 100  | \~700      | 10,000 | 1e30+ |
| 1000 | 1    | \~10     | 1000 | \~10,000   | 1M     | no ❤️ |

---

## 🔥 Survival Tips

1. **Don’t trust nested loops** unless you _really_ have to.
2. **`log n` is your bestie** — it's surprisingly powerful.
3. **Use HashMaps wisely** for constant time lookups.
4. **Don’t fear Big O** — it’s a vibe check, not a death sentence.
5. **Scaling is about patterns**, not lines of code.

---

### 🌈 Bonus: Izzy’s Runtime Radar

| Feels like...       | It's probably...    | Should you panic?   |
| ------------------- | ------------------- | ------------------- |
| Fast no matter what | O(1), O(log n)      | 💅 Not today, Satan |
| Slow as list grows  | O(n), O(n log n)    | 😬 Monitor it       |
| Explodes quickly    | O(n²), O(2ⁿ), O(n!) | 🚨 Abort mission    |

---
