use std::time::Instant;

// Logs the time taken by a function or block
pub fn log_time<F, R>(description: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f(); // Execute the function
    println!("{} completed in {:?} ms", description, start.elapsed().as_millis());
    result
}