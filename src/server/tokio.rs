use super::utils::*;
use tokio::net::{TcpListener, TcpStream};

pub async fn main() -> crate::Result {
    let listener = TcpListener::bind(ADDR).await?;
    debug!(ADDR, "Listening on");

    let (sender, mut receiver) = channel::<Message>(1024);
    let mut task_stat = Some(stat(sender));

    loop {
        tokio::select! {
            request = listener.accept() => {
                let (socket, socket_addr) = request?;
                if let Some(stat) = task_stat.take() {
                    tokio::spawn(stat);
                }
                tokio::spawn(respond(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return Ok(()); }
            }
        }
    }
}

async fn respond(mut socket: TcpStream, socket_addr: SocketAddr) {
    let span = error_span!("respond", %socket_addr);
    async move {
        let mut buf = vec![0; SIZE];

        loop {
            let n = match socket.read(&mut buf).await {
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
        }
    }
    .instrument(span)
    .await
}
