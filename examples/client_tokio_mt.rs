use std::error::Error;
use std::io::ErrorKind;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::task::JoinSet;

const ADDR: &str = "127.0.0.1:2345";
/// How many bytes to be transmitted.
const SIZE: usize = 16 * 1024;
/// Real data to be transmitted.
const DATA: &[u8] = &[0; SIZE];
/// How many socket to connect to the server.
const LEN: usize = 100;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut v_stream = Vec::with_capacity(LEN);
    for _ in 0..LEN {
        v_stream.push(TcpStream::connect(ADDR).await?);
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
                            ErrorKind::BrokenPipe | ErrorKind::ConnectionReset
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
