mod cli;
use cli::*;

fn main() -> bench_uring::Result {
    bench_uring::logger::init();

    let examples = Examples::new()?;

    examples.build()?;
    let throughputs = examples.bench()?;
    dbg!(throughputs);

    Ok(())
}
