#[tokio::main(flavor = "current_thread")]
async fn main() -> bench_uring::Result {
    bench_uring::server::async_uring::main().await
}
