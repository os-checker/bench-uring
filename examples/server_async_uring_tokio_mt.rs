#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> bench_uring::Result {
    bench_uring::server::async_uring::main().await
}
