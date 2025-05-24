mod cli;
use cli::*;

#[macro_use]
extern crate tracing;

fn main() -> bench_uring::Result {
    bench_uring::logger::init();

    let args: Vec<_> = std::env::args().collect();
    // The first argument is a path to csv.
    let csv = args.get(1).map(|s| s.as_str()).unwrap_or("data.csv");

    let examples = Examples::new()?;

    examples.build()?;
    let throughputs = examples.bench()?;
    dbg!(&throughputs);
    write_csv(csv, &throughputs)?;

    Ok(())
}
