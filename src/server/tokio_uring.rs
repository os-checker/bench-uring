use tokio_uring::net::{TcpListener, TcpStream};

use super::utils::*;

pub async fn main() -> crate::Result {
    let addr = ADDR.parse()?;
    let listener = TcpListener::bind(addr)?;
    println!("Listening on: {ADDR}");

    let (sender, mut receiver) = channel::<Message>(1024);
    let mut task_stat = Some(stat(sender));

    loop {
        tokio::select! {
            request = listener.accept() => {
                let (socket, socket_addr) = request?;
                if let Some(stat) = task_stat.take() {
                    tokio_uring::spawn(stat);
                }
                tokio_uring::spawn(response(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return Ok(()); }
            }
        }
    }
}

async fn response(socket: TcpStream, socket_addr: SocketAddr) {
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
