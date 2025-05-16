use bench_uring::utils::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result {
    bench_uring::tokio::main().await
}
