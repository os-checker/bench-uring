// With multi-thread function can not have a return value
#[monoio::main(worker_threads = 8, enable_timer = true)]
async fn main() {
    bench_uring::server::monoio::main().await
}
