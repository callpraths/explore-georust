use serde::Serialize;
use std::{
    thread,
    time::{Duration, Instant, SystemTime},
};

pub mod simple;

pub trait Program {
    fn name(&self) -> String;
    fn prepare(&self) -> Box<dyn PreparedProgram>;
}

pub trait PreparedProgram {
    fn benchmark_this(&mut self);
}

pub struct Args {
    pub programs: Vec<Box<dyn Program>>,
    pub iterations: usize,
    pub discard_leading: Option<usize>,
    pub pause: Option<Duration>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BenchmarkResult {
    pub iterations: Vec<Iteration>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Iteration {
    pub i: usize,
    pub started_at: SystemTime,
    pub runs: Vec<Run>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Run {
    pub program: String,
    pub duration: Duration,
}

pub fn benchmark_run(args: Args) -> BenchmarkResult {
    let mut result: BenchmarkResult = BenchmarkResult {
        iterations: Vec::with_capacity(args.iterations),
    };

    let radix = args.programs.len();
    let mut head: usize = 0;
    for i in 0..args.iterations {
        let mut iteration = Iteration {
            i,
            started_at: SystemTime::now(),
            runs: Vec::with_capacity(args.programs.len()),
        };

        for p in 0..radix {
            iteration
                .runs
                .push(run_once(&args.programs[(head + p) % radix]));
        }
        head = (head + 1) % radix;

        if should_report_iteration(i, &args) {
            result.iterations.push(iteration);
            println!("REPPORTED  Iteration {} of {}", i + 1, args.iterations);
        } else {
            println!("SKIPPED    Iteration {} of {}", i + 1, args.iterations);
        }

        if let Some(pause) = args.pause {
            println!("");
            println!("Pausing for {:#?} ...", pause);
            thread::sleep(pause);
            println!("");
        }
    }
    result
}

fn should_report_iteration(iteration: usize, args: &Args) -> bool {
    match args.discard_leading {
        Some(value) => iteration >= value,
        None => true,
    }
}

fn run_once(program: &Box<dyn Program>) -> Run {
    let mut prepared = program.prepare();

    let start = Instant::now();
    prepared.benchmark_this();
    let end = Instant::now();
    Run {
        program: program.name(),
        duration: end - start,
    }
}
