mod cli;
use cli::*;

fn main() -> bench_uring::Result {
    let examples = Examples::new()?;

    examples.build()?;

    Ok(())
}
