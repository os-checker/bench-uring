mod cli;
use cli::*;

#[macro_use]
extern crate tracing;

fn main() -> bench_uring::Result {
    bench_uring::logger::init();

    let examples = Examples::new()?;

    examples.build()?;
    let throughputs = examples.bench()?;
    dbg!(&throughputs);
    write_csv("data.csv", &throughputs)?;

    Ok(())
}
