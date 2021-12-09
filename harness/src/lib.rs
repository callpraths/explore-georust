pub mod notsofine {
    use serde::Serialize;
    use std::time::{Duration, Instant, SystemTime};

    pub trait Program {
        fn name(&self) -> String;
        fn prepare(&self) -> Box<dyn PreparedProgram>;
    }

    pub trait PreparedProgram {
        fn benchmark_this(&self);
    }

    pub struct Args {
        pub programs: Vec<Box<dyn Program>>,
        pub iterations: usize,
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct Iteration {
        pub program: String,
        pub started_at: SystemTime,
        pub duration: Duration,
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct BenchmarkResult {
        pub runs: Vec<Iteration>,
    }

    pub fn benchmark_run(args: Args) -> BenchmarkResult {
        let mut result: BenchmarkResult = BenchmarkResult {
            runs: Vec::with_capacity(args.iterations * args.programs.len()),
        };

        let radix = args.programs.len();
        let mut head: usize = 0;
        for i in 0..args.iterations {
            println!("Iteration {} of {}", i, args.iterations);
            for i in 0..radix {
                result
                    .runs
                    .push(run_once(&args.programs[(head + i) % radix]));
            }
            head = (head + 1) % radix;
        }
        result
    }

    fn run_once(program: &Box<dyn Program>) -> Iteration {
        let prepared = program.prepare();
        let wall_clock_start = SystemTime::now();
        let start = Instant::now();
        prepared.benchmark_this();
        let end = Instant::now();
        Iteration {
            program: program.name(),
            started_at: wall_clock_start,
            duration: end - start,
        }
    }
}
