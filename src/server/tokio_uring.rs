use tokio_uring::net::{TcpListener, TcpStream};

use super::utils::*;

pub fn start() -> crate::Result {
    tokio_uring::start(main())
}

pub async fn main() -> crate::Result {
    let addr = ADDR.parse()?;
    let listener = TcpListener::bind(addr)?;
    debug!(ADDR, "Listening on");

    let (sender, mut receiver) = channel::<Message>(1024);
    let mut task_stat = Some(stat(sender));

    loop {
        tokio::select! {
            request = listener.accept() => {
                let (socket, socket_addr) = request?;
                if let Some(stat) = task_stat.take() {
                    tokio_uring::spawn(stat);
                }
                tokio_uring::spawn(respond(socket, socket_addr));
            }
            recv = receiver.recv() => {
                if recv.is_none() { return Ok(()); }
            }
        }
    }
}

async fn respond(socket: TcpStream, socket_addr: SocketAddr) {
    let _span = error_span!("respond", %socket_addr).entered();
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
