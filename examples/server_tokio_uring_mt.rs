use bench_uring::server::tokio_uring::start;

/// All multi-thread async runtimes are using 8 threads in total.
const THREAD_NUMS: u8 = 8;

fn main() -> bench_uring::Result {
    let handles: Vec<_> = (1..THREAD_NUMS)
        .map(|_| std::thread::spawn(start))
        .collect();

    start()?;

    for handle in handles {
        handle.join().unwrap()?;
    }
    Ok(())
}
