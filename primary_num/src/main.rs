use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

// const NUM_THREADS: usize = 10;
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
    let num_cpus = num_cpus::get();
    println!("Using {num_cpus} threads.");
    let candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We are not timing the initial condition
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));
    // let primes: Vec<usize> = candidates
    //     .iter()
    //     .filter(|n| is_prime(**n))
    //     .map(|n| *n)
    //     .collect();

    std::thread::scope(|scope| {
        let chunks = candidates.chunks(candidates.len() / num_cpus);

        for chunk in chunks {
            let my_primes = primes.clone();
            scope.spawn(move || {
                // Perform the same filter/map/collect chain as we did single-threaded
                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();

                // Lock the shared results list
                let mut lock = my_primes.lock().unwrap();

                // Extend the results with our discovered primes
                lock.extend(local_results);
            });
        }

        // The scope will automatically wait for child threads to finish here.
    });

    // Time how long it took
    let elapsed = start.elapsed();

    // Results
    let lock = primes.lock().unwrap();
    println!("Found {} primes", lock.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}
