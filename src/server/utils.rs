use std::sync::atomic::AtomicUsize;

pub use std::net::SocketAddr;
pub use std::sync::atomic::Ordering;
pub use std::time::{Duration, Instant};

pub use tokio::io::AsyncReadExt;
pub use tokio::sync::mpsc::{Sender, channel};

pub static COUNT: AtomicUsize = AtomicUsize::new(0);

pub const ADDR: &str = "127.0.0.1:2345";
/// How many bytes to be transmitted.
pub const SIZE: usize = 16 * 1024;
/// How long the server last.
pub const DURATION: Duration = Duration::from_secs(10);

pub enum Message {
    StatDone,
}

pub async fn stat(sender: Sender<Message>) {
    let start = Instant::now();
    let mut last = 0;
    let mut last_time = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(2));

    interval.tick().await;

    while start.elapsed() < DURATION {
        let time = interval.tick().await.into_std();

        let val = COUNT.load(Ordering::Relaxed);
        let cnt = val - last;
        last = val;

        let duration = time - last_time;
        last_time = time;

        let amt = cnt as u64 / duration.as_secs();
        println!("Req/sec: {amt} ({cnt} / {duration:.0?})");
    }

    let duration = start.elapsed();
    let amt = last as u64 / duration.as_secs();
    println!("Avg: {amt} ({last} / {duration:.0?})");
    if let Err(err) = sender.send(Message::StatDone).await {
        eprintln!("[stat] {err}");
    }
}
