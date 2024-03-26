use std::time::Instant;

const MAX_NUMBER: usize = 100_000;

/// Really inefficient prime number calculator
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        false
    } else {
        for div in 2..n {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We are not timing the initial condition
    let primes: Vec<usize> = candidates
        .iter()
        .filter(|n| is_prime(**n))
        .map(|n| *n)
        .collect();
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
