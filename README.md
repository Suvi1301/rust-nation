# rust-nation
https://www.rustnationuk.com/workshops


PART 1: RUST'S SUPERPOWERS
Discover the unique strengths of Rust and the mechanisms that make them possible.

Fearless Concurrency & High-Performance Seatbelts
Understanding Rust's concurrency promises
The role and implications of the `unsafe` keyword.
Mutexes in Rust: How they differ from other platforms.
Real-world implications: Uber's data race challenges.
The Super-Power of Drop
Introduction to the Drop trait and its significance.
Practical demonstrations of Drop in action.
Memory management in Rust: The magic behind it.
The role of Rc and Arc in memory management.
How libraries leverage Rust's memory management features.
Hands-on Exercise
Dive into threading with a concise demo program.

PART 2: ASYNCHRONOUS PROGRAMMING
Demystifying async in Rust and harnessing its potential.

Understanding Async vs. Threads
The evolution and current state of async in Rust.
Key differences between async and threads.
The benefits of lightweight, cooperatively scheduled async tasks.
The ideal use cases for threads vs. async.
Introduction to various async runtimes: Tokio, smol, glommer. Tokio:
The Heart of Rust's Async
Building a REST server with Tokio, Serde, SQLX, and Axum.
Introduction to Tower layer as a shared database resource.
The power and efficiency of caching in Rust.
Real-world performance metrics and implications.
Efficiently handling thousands of requests with minimal resources.
Hands-on Exercise
Construct a caching REST server and witness Rust's efficiency.

PART 3: THREADS AND ASYNC, HARMONIOUSLY COEXISTING
Explore the seamless integration of threaded and async worlds in Rust.

Threaded vs. Async: Myths Debunked
Running multiple runtimes and their benefits.
Communication between runtimes using MPSC channels.
The role and function of `tokio::main`
Customizing runtimes with Tokio's RuntimeBuilder and “block_on”.
The benefits of controlling resource allocation across runtimes
Mixing Domains for Maximum Efficiency
Practical demonstrations of combining threaded and async tasks.
The power of Tokio's `spawn blocking`
Communication between threaded and async domains.
Building efficient systems with network services in Rust.
The potential and benefits of read-write lock-protected storage.
