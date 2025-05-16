#[monoio::main(enable_timer = true)]
async fn main() {
    bench_uring::server::monoio::main().await
}
