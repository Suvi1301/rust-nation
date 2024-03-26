use rand::seq::SliceRandom;
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

fn main_without_mutex() {
    let num_cpus = num_cpus::get();
    println!("Using {num_cpus} threads.");
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We are not timing the initial condition
    let mut primes: Vec<usize> = Vec::with_capacity(10_000);

    // Shuffle the candidates so we can randomise the slices for each thread.
    candidates.shuffle(&mut rand::thread_rng());

    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(num_cpus);
        let chunks = candidates.chunks(num_cpus); // chunks at a time. So if 16 cpus, then 16 items per list.

        for chunk in chunks {
            let handle =
                scope.spawn(move || chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect());
            handles.push(handle);
        }

        for handle in handles {
            let local_result: Vec<usize> = handle.join().unwrap();
            primes.extend(local_result);
        }
    });

    // Time how long it took
    let elapsed = start.elapsed();

    // Results
    println!("Found {} primes", primes.len());
    println!("Calculated in {:.4} seconds", elapsed.as_secs_f32());
}

fn main() {
    let num_cpus = num_cpus::get();
    println!("Using {num_cpus} threads.");
    let mut candidates: Vec<usize> = (0..MAX_NUMBER).collect();

    // Perform the calculation
    let start = Instant::now(); // We are not timing the initial condition
    let primes: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    // Shuffle the candidates so we can randomise the slices for each thread.
    candidates.shuffle(&mut rand::thread_rng());

    std::thread::scope(|scope| {
        let chunks = candidates.chunks(num_cpus); // chunks at a time. So if 16 cpus, then 16 items per list.

        for (id, chunk) in chunks.enumerate() {
            let my_primes = primes.clone();
            scope.spawn(move || {
                // Time each chunk
                let chunk_start = Instant::now();

                // Perform the same filter/map/collect chain as we did single-threaded
                let local_results: Vec<usize> =
                    chunk.iter().filter(|n| is_prime(**n)).map(|n| *n).collect();

                // Lock the shared results list
                let mut lock = my_primes.lock().unwrap();

                // Extend the results with our discovered primes
                lock.extend(local_results);

                // Print the time for each chunk
                let chunk_elapsed = chunk_start.elapsed();
                println!(
                    "Thread #{id} took {:.3} seconds",
                    chunk_elapsed.as_secs_f32()
                );
                // Notice the threads with higher id will take longer because they have to work harder on the chunks
                // because the chunks are slices and the later chunks have harder numbers to crack for is_prime.
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
