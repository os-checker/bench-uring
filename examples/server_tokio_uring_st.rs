fn main() -> bench_uring::Result {
    bench_uring::server::tokio_uring::start()
}
