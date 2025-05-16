fn main() -> bench_uring::Result {
    tokio_uring::start(bench_uring::server::tokio_uring::main())
}
