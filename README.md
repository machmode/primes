# primes

A Rust workspace for generating and visualising prime numbers.

## Crates

### mrbmsieve

Combines two prime-finding algorithms with three visualisation outputs.

For different use cases, there are more efficient approaches:

For checking a single large number: The Miller-Rabin primality test is much faster — O(k·log²n) vs O(√n) for trial division. It's probabilistic but can be made deterministic for numbers below 3.3×10²⁴ using specific witness sets.
For generating all primes up to N: The Sieve of Eratosthenes is quite good, but there are improvements:

- Segmented Sieve — cache-friendly, uses O(√n) memory instead of O(n)
- Sieve of Atkin — theoretically O(n/log log n), though in practice often similar to Eratosthenes
- Bit-packed sieve skipping evens — halves memory and improves cache performance

This implementation is an optimized version with Miller-Rabin and a bit-packed sieve:

#### Algorithms

- **Sieve of Eratosthenes** -- Bit-packed sieve storing only odd numbers, halving memory usage. Generates all primes up to a configurable limit (set via the `max` variable in `main()`).

- **Deterministic Miller-Rabin** -- Primality test using 12 fixed witnesses (`2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37`), sufficient to give correct results for all `u64` values. Used to test individual large numbers beyond the sieve range.

#### Plot outputs

All plots are saved to a `plots/` subdirectory as PNG images, rendered white-on-black.

- **Prime Gaps** (`plots/prime_gaps.png`, 1600x900) -- Scatter plot of the gap between consecutive primes (`p[i+1] - p[i]`) against the prime `p[i]`. Downsampled to 100,000 points maximum when the prime count is large.

- **Sacks Spiral** (`plots/sacks_spiral.png`, 1600x1600) -- Each prime `p` is placed at polar coordinates `(sqrt(p), 2*pi*sqrt(p))`, converted to Cartesian for plotting. Primes up to 500,000 are included. Reveals curved lines where primes cluster along quadratic polynomials.

- **Ulam Spiral** (`plots/ulam_spiral.png`, 1600x1600) -- Integers are laid out in a spiral grid starting from the centre; primes are highlighted as pixels. Primes up to 1,000,000 are included (producing a ~1001x1001 grid). Shows the well-known diagonal-line clustering of primes.

#### Console output

The program prints:
- Total count of primes found by the sieve
- The first 20 and last 5 primes
- Miller-Rabin primality checks on a few large numbers

## Building and running

```sh
cargo build --release
cargo run --release -p mrbmsieve
```

## Dependencies

- [plotters](https://crates.io/crates/plotters) -- chart rendering
