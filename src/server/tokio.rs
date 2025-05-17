use super::utils::*;

use tokio::net::{TcpListener, TcpStream};

pub async fn main() -> crate::Result {
    let listener = TcpListener::bind(ADDR).await?;
    // println!("Listening on: {ADDR}");

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
    // println!("new client: {socket_addr}");
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
            // println!("close client: {socket_addr}");
            return;
        }
    }
}
