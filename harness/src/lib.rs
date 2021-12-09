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
            println!("Iteration {} of {}", i, args.iterations);
            for p in 0..radix {
                iteration
                    .runs
                    .push(run_once(&args.programs[(head + p) % radix]));
            }
            head = (head + 1) % radix;
            result.iterations.push(iteration);
        }
        result
    }

    fn run_once(program: &Box<dyn Program>) -> Run {
        let prepared = program.prepare();

        let start = Instant::now();
        prepared.benchmark_this();
        let end = Instant::now();
        Run {
            program: program.name(),
            duration: end - start,
        }
    }
}
