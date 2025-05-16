use bench_uring::utils::Result;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result {
    bench_uring::tokio::main().await
}
