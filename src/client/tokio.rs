use crate::utils::*;
use std::io::ErrorKind;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task::JoinSet;

pub async fn main() -> Result {
    let mut v_stream = Vec::with_capacity(LEN);
    for _ in 0..LEN {
        loop {
            match TcpStream::connect(ADDR).await {
                Ok(stream) => {
                    v_stream.push(stream);
                    break;
                }
                Err(err) => eprintln!("{err}"),
            }
        }
    }

    let mut set = JoinSet::new();
    for mut stream in v_stream {
        set.spawn(async move {
            loop {
                let result = stream.write_all(DATA).await;
                match result {
                    Ok(()) => (),
                    Err(err) => {
                        if !matches!(
                            err.kind(),
                            ErrorKind::BrokenPipe
                                | ErrorKind::ConnectionReset
                                | ErrorKind::ConnectionRefused
                        ) {
                            eprintln!("Failed to write data: {err}");
                        }
                        break;
                    }
                }
            }
        });
    }

    _ = set.join_all().await;

    Ok(())
}
