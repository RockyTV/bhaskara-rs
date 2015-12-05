# bhaskara-rs
**bhaskara-rs** is an application written in [Rust][rust] that solves quadratic equations (ax^2 + bx + c = 0) using the [quadratic formula][formula] (x = -b +- sqroot(b^2 - 4ac) / 2a).

### Compiling
bhaskara-rs was written in Rust nightly, therefore compiling it with Rust stable or unstable might not work.
```bash
git clone https://github.com/RockyTV/bhaskara-rs
cd bhaskara-rs
cargo build
```

### Sample output
Input:
```bash
Input A:
x^2
Input B:
+10x
Input C:
25
```
Output:
`x is (-5, -5)`

[rust]: https://rust-lang.org/
[formula]: https://en.wikipedia.org/wiki/Quadratic_formula
