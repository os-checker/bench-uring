use monoio::{
    io::AsyncReadRent,
    net::{TcpListener, TcpStream},
};

use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{Sender, channel};

const ADDR: &str = "127.0.0.1:2345";
/// How many bytes to be transmitted.
const SIZE: usize = 16 * 1024;
/// How long the server last.
const DURATION: Duration = Duration::from_secs(10);

// With multi-thread function can not have a return value
#[monoio::main(worker_threads = 8, enable_timer = true)]
async fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("Listening on: {ADDR}");

    // If channel is commucated across threads, monoio's "sync" feature
    // should be enabled.
    let (sender, mut receiver) = channel::<Message>(1024);
    let mut task_stat = Some(stat(sender));

    loop {
        monoio::select! {
            request = listener.accept() => {
                let (socket, socket_addr) = request.unwrap();
                if let Some(stat) = task_stat.take() {
                    monoio::spawn(stat);
                }
                monoio::spawn(response(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return; }
            }
        }
    }
}

async fn response(mut socket: TcpStream, socket_addr: SocketAddr) {
    println!("new client: {socket_addr}");
    let mut buf = vec![0; SIZE];
    let mut res;

    loop {
        (res, buf) = socket.read(buf).await;

        let n = match res {
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

        // clear
        buf.clear();
    }
}

static COUNT: AtomicUsize = AtomicUsize::new(0);

enum Message {
    StatDone,
}

async fn stat(sender: Sender<Message>) {
    let start = Instant::now();
    let mut last = 0;
    let mut last_time = Instant::now();
    let mut interval = monoio::time::interval(Duration::from_secs(2));

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
