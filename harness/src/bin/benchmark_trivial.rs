use clap::Parser;
use harness::notsofine::*;
use std::{fs::File, io::Write, thread, time::Duration};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CLIArgs {
    /// Name of the person to greet
    #[clap(short, long)]
    out_file: String,

    #[clap(short, long)]
    iterations: usize,
}

fn sleep_some() {
    thread::sleep(Duration::from_millis(100));
}

fn loop_some() {
    for _ in 1..40000 {
        for _ in 1..100 {}
    }
}

fn main() {
    let args = CLIArgs::parse();
    let result = benchmark_run(Args {
        programs: vec![
            simple::program_for_fn("Looper1", loop_some),
            simple::program_for_fn("Sleerp", sleep_some),
            simple::program_for_fn("Looper2", loop_some),
            simple::program_for_fn("Looper3", loop_some),
        ],
        iterations: args.iterations,
    });

    let mut ofile = File::create(args.out_file).unwrap();
    ofile
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
