use super::utils::*;

use monoio::{
    io::AsyncReadRent,
    net::{TcpListener, TcpStream},
};

pub async fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    debug!(ADDR, "Listening on");

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
                monoio::spawn(respond(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return; }
            }
        }
    }
}

async fn respond(mut socket: TcpStream, socket_addr: SocketAddr) {
    let span = error_span!("respond", %socket_addr);
    async move {
        let mut buf = vec![0; SIZE];
        let mut res;

        loop {
            (res, buf) = socket.read(buf).await;

            let n = match res {
                Ok(n) => n,
                Err(err) => {
                    error!(?err, "Failed to read socket.");
                    break;
                }
            };

            COUNT.fetch_add(1, Ordering::Relaxed);
            if n == 0 {
                debug!("Close client");
                return;
            }

            // clear
            buf.clear();
        }
    }
    .instrument(span)
    .await
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
        debug!("Req/sec: {amt} ({cnt} / {duration:.0?})");
    }

    let duration = start.elapsed();
    let amt = last as u64 / duration.as_secs();
    println!("Avg: {amt} ({last} / {duration:.0?})");
    if let Err(err) = sender.send(Message::StatDone).await {
        error!(?err, "Failed to send a message.");
    }
}
