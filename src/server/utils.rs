use std::sync::atomic::AtomicUsize;

pub use crate::utils::*;
pub use std::{net::SocketAddr, sync::atomic::Ordering, time::Instant};
pub use tokio::{
    io::AsyncReadExt,
    sync::mpsc::{Sender, channel},
};

pub static COUNT: AtomicUsize = AtomicUsize::new(0);

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
        debug!("Req/sec: {amt} ({cnt} / {duration:.0?})");
    }

    let duration = start.elapsed();
    let amt = last as u64 / duration.as_secs();
    println!("Avg: {amt} ({last} / {duration:.0?})");
    if let Err(err) = sender.send(Message::StatDone).await {
        error!(?err, "Failed to send a message.");
    }
}
