mod cli;
use cli::*;

fn main() -> bench_uring::Result {
    let examples = examples()?;

    for example in &examples {
        run("cargo", &["build", "--example", example], |_| Ok(()))?;
    }

    Ok(())
}
