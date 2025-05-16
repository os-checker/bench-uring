#[tokio::main(flavor = "current_thread")]
async fn main() -> bench_uring::Result {
    bench_uring::server::tokio::main().await
}
