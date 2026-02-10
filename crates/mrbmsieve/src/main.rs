use plotters::prelude::*;

/// Deterministic Miller-Rabin for all u64 values
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp >>= 1;
        base = base * base % modulus;
    }
    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    // Deterministic witnesses sufficient for all u64
    let witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d >>= 1;
        r += 1;
    }

    let n128 = n as u128;
    'witness: for &a in &witnesses {
        if a >= n { continue; }
        let mut x = mod_pow(a as u128, d as u128, n128);
        if x == 1 || x == n128 - 1 { continue; }
        for _ in 0..r - 1 {
            x = x * x % n128;
            if x == n128 - 1 { continue 'witness; }
        }
        return false;
    }
    true
}

/// Bit-packed sieve of odd numbers only — halves memory usage
fn sieve(limit: usize) -> Vec<usize> {
    if limit < 2 { return vec![]; }
    let half = limit / 2; // index i represents number 2*i + 1
    let mut bits = vec![true; half];
    // bits[0] = true represents 1, but we skip it in output

    let mut i = 1; // start at 3
    while (2 * i + 1) * (2 * i + 1) <= limit {
        if bits[i] {
            // mark odd multiples of (2i+1) starting at (2i+1)²
            let prime = 2 * i + 1;
            let mut j = 2 * i * (i + 1); // index of prime²
            while j < half {
                bits[j] = false;
                j += prime;
            }
        }
        i += 1;
    }

    let mut primes = vec![2];
    for i in 1..half {
        if bits[i] {
            primes.push(2 * i + 1);
        }
    }
    primes
}

fn fmt_num(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result
}

fn plot_gaps(primes: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let max_points = 100_000;
    let step = (primes.len() / max_points).max(1);

    let gaps: Vec<(f64, f64)> = primes
        .windows(2)
        .step_by(step)
        .map(|w| (w[0] as f64, (w[1] - w[0]) as f64))
        .collect();

    println!("Plotting {} of {} gaps (1:{} sample)", gaps.len(), primes.len() - 1, step);

    let x_max = *primes.last().unwrap() as f64;
    let y_max = gaps.iter().map(|&(_, g)| g).fold(0.0f64, f64::max);

    let root = BitMapBackend::new("plots/prime_gaps.png", (1600, 900)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Prime Gaps", ("sans-serif", 30).into_font().color(&WHITE))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0f64..x_max, 0f64..y_max + 2.0)?;

    chart
        .configure_mesh()
        .x_desc("Prime p")
        .y_desc("Gap to next prime")
        .axis_style(WHITE)
        .label_style(("sans-serif", 14).into_font().color(&WHITE))
        .bold_line_style(WHITE.mix(0.2))
        .light_line_style(WHITE.mix(0.05))
        .draw()?;

    chart.draw_series(
        gaps.iter().map(|&(x, y)| Circle::new((x, y), 1, WHITE.mix(0.6).filled())),
    )?;

    root.present()?;
    println!("Plot saved to plots/prime_gaps.png");
    Ok(())
}

fn plot_sacks(primes: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let sacks_limit = 500_000.min(*primes.last().unwrap());
    let tau = 2.0 * std::f64::consts::PI;

    let points: Vec<(f64, f64)> = primes
        .iter()
        .take_while(|&&p| p <= sacks_limit)
        .map(|&p| {
            let r = (p as f64).sqrt();
            let theta = r * tau;
            (r * theta.cos(), r * theta.sin())
        })
        .collect();

    let bound = (sacks_limit as f64).sqrt() + 1.0;

    let root = BitMapBackend::new("plots/sacks_spiral.png", (1600, 1600)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sacks Spiral", ("sans-serif", 30).into_font().color(&WHITE))
        .margin(20)
        .build_cartesian_2d(-bound..bound, -bound..bound)?;

    chart.draw_series(
        points.iter().map(|&(x, y)| Circle::new((x, y), 1, WHITE.mix(0.6).filled())),
    )?;

    root.present()?;
    println!(
        "Sacks spiral saved to plots/sacks_spiral.png ({} primes up to {})",
        points.len(),
        fmt_num(sacks_limit),
    );
    Ok(())
}

fn plot_ulam(primes: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let ulam_limit = 1_000_000.min(*primes.last().unwrap());
    let side = ((ulam_limit as f64).sqrt().ceil()) as usize | 1; // ensure odd
    let half = side as i32 / 2;

    // Build a set of primes up to ulam_limit for fast lookup
    let prime_set: std::collections::HashSet<usize> = primes
        .iter()
        .take_while(|&&p| p <= ulam_limit)
        .copied()
        .collect();

    // Walk the Ulam spiral: start at centre, move right, up, left, down...
    // Step pattern: R1, U1, L2, D2, R3, U3, L4, D4, ...
    let dirs: [(i32, i32); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut grid = vec![vec![false; side]; side];
    let (mut x, mut y) = (half, half);
    let mut n: usize = 1;
    let mut step_size = 1;
    let mut dir_idx = 0;
    let mut steps_taken = 0;
    let mut turns = 0;

    while n <= ulam_limit {
        let ux = x as usize;
        let uy = y as usize;
        if ux < side && uy < side && prime_set.contains(&n) {
            grid[uy][ux] = true;
        }
        n += 1;

        let (dx, dy) = dirs[dir_idx];
        x += dx;
        y += dy;
        steps_taken += 1;

        if steps_taken == step_size {
            steps_taken = 0;
            dir_idx = (dir_idx + 1) % 4;
            turns += 1;
            if turns % 2 == 0 {
                step_size += 1;
            }
        }
    }

    let root = BitMapBackend::new("plots/ulam_spiral.png", (1600, 1600)).into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Ulam Spiral", ("sans-serif", 30).into_font().color(&WHITE))
        .margin(20)
        .build_cartesian_2d(0..side, 0..side)?;

    let mut points = Vec::new();
    for (row, cols) in grid.iter().enumerate() {
        for (col, &is_prime) in cols.iter().enumerate() {
            if is_prime {
                points.push((col, row));
            }
        }
    }

    chart.draw_series(
        points.iter().map(|&(x, y)| Pixel::new((x, y), WHITE.mix(0.8))),
    )?;

    root.present()?;
    println!(
        "Ulam spiral saved to plots/ulam_spiral.png ({} primes up to {}, {}x{} grid)",
        prime_set.len(),
        fmt_num(ulam_limit),
        side,
        side,
    );
    Ok(())
}

fn main() {
    std::fs::create_dir_all("plots").expect("Failed to create plots directory");

    let max = 10_000_000;
    let primes = sieve(max);
    println!("Primes up to {}: {} found", fmt_num(max), fmt_num(primes.len()));
    let first20: Vec<String> = primes[..20].iter().map(|&p| fmt_num(p)).collect();
    println!("First 20: [{}]", first20.join(", "));
    let last5: Vec<String> = primes[primes.len()-5..].iter().map(|&p| fmt_num(p)).collect();
    println!("Last 5:   [{}]", last5.join(", "));

    println!("\nMiller-Rabin checks:");
    for n in [999_999_999_989u64, 1_000_000_007, 2u64.pow(61) - 1] {
        println!("  {n} → {}prime", if is_prime(n) { "" } else { "not " });
    }
    // println!("Primes: {:?}", primes);

    plot_gaps(&primes).expect("Failed to generate gap plot");
    plot_sacks(&primes).expect("Failed to generate Sacks spiral");
    plot_ulam(&primes).expect("Failed to generate Ulam spiral");
}