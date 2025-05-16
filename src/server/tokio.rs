use super::utils::*;

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{Sender, channel};

pub async fn main() -> crate::Result {
    let listener = TcpListener::bind(ADDR).await?;
    println!("Listening on: {ADDR}");

    let (sender, mut receiver) = channel::<Message>(1024);
    let mut task_stat = Some(stat(sender));

    loop {
        tokio::select! {
            request = listener.accept() => {
                let (socket, socket_addr) = request?;
                if let Some(stat) = task_stat.take() {
                    tokio::spawn(stat);
                }
                tokio::spawn(response(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return Ok(()); }
            }
        }
    }
}

async fn response(mut socket: TcpStream, socket_addr: SocketAddr) {
    println!("new client: {socket_addr}");
    let mut buf = vec![0; SIZE];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(n) => n,
            Err(err) => {
                eprintln!("{socket_addr}: {err}");
                break;
            }
        };

        COUNT.fetch_add(1, Ordering::Relaxed);
        if n == 0 {
            println!("close client: {socket_addr}");
            return;
        }
    }
}

enum Message {
    StatDone,
}

async fn stat(sender: Sender<Message>) {
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
