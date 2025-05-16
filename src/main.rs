use bench_uring::Result;
use std::process::Command;

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
        run("cargo", &["build", "--example", example], |stdout| {
            println!("{stdout}");
            Ok(())
        })?;
    }

    Ok(())
}

fn run<T>(exe: &str, args: &[&str], f: impl FnOnce(String) -> Result<T>) -> Result<T> {
    let output = Command::new(exe)
        .args(args)
        .stdout(std::io::stdout())
        .stderr(std::io::stderr())
        .output()?;
    if !output.status.success() {
        return Err(format!("Failed to run {exe:?} {args:?}",).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    f(stdout)
}
