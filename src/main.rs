use bench_uring::Result;
use std::{io, process::Command};

fn main() -> Result {
    let examples = [
        "client_tokio_mt",
        "server_monoio_mt",
        "server_tokio_mt",
        "server_tokio_uring_mt",
        "client_tokio_st",
        "server_monoio_st",
        "server_tokio_st",
        "server_tokio_uring_st",
    ];

    for example in examples {
        run("cargo", &["build", "--example", example], |_| Ok(()))?;
    }

    Ok(())
}

fn run<T>(exe: &str, args: &[&str], f: impl FnOnce(String) -> Result<T>) -> Result<T> {
    use io::{Read, Write};

    let (mut reader, writer) = io::pipe()?;
    let mut cmd = Command::new(exe)
        .args(args)
        .stdout(writer.try_clone()?)
        .stderr(writer)
        .spawn()?;

    let mut buf = Vec::new();

    _ = reader.read_to_end(&mut buf)?;
    io::stdout().write_all(&buf)?;

    if !cmd.wait()?.success() {
        return Err(format!("Failed to run {exe:?} {args:?}",).into());
    }
    let stdout = String::from_utf8(buf)?;
    f(stdout)
}
