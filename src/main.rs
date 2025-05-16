use bench_uring::Result;
use std::{io, process::Command};

fn main() -> Result {
    let examples = examples()?;

    for example in &examples {
        run("cargo", &["build", "--example", example], |_| Ok(()))?;
    }

    Ok(())
}

fn examples() -> Result<Vec<String>> {
    let mut v = Vec::new();
    for entry in std::fs::read_dir("examples")? {
        let entry = entry?;
        let path = entry.path();
        if entry.metadata()?.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false) {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                v.push(file_stem.to_owned());
            }
        }
    }
    Ok(v)
}

fn run<T>(exe: &str, args: &[&str], f: impl FnOnce(String) -> Result<T>) -> Result<T> {
    use io::{Read, Write};

    let stdout = &mut io::stdout();
    writeln!(stdout, "{exe:?} {args:?}")?;

    let (mut reader, writer) = io::pipe()?;
    let mut cmd = Command::new(exe)
        .args(args)
        .stdout(writer.try_clone()?)
        .stderr(writer)
        .spawn()?;

    let mut buf = Vec::new();

    _ = reader.read_to_end(&mut buf)?;
    stdout.write_all(&buf)?;

    if !cmd.wait()?.success() {
        return Err(format!("Failed to run {exe:?} {args:?}",).into());
    }
    let stdout = String::from_utf8(buf)?;
    f(stdout)
}
