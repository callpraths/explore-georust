pub mod notsofine {
    use std::time::{Duration, Instant, SystemTime};

    pub trait Program {
        type P: PreparedProgram;
        fn prepare(&self) -> Self::P;
    }

    pub trait PreparedProgram {
        fn benchmark_this(self);
    }

    pub struct Args<P: Program> {
        pub p: P,
        pub iterations: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Iteration {
        pub started_at: SystemTime,
        pub duration: Duration,
    }

    #[derive(Clone, Debug)]
    pub struct BenchmarkResult {
        pub runs: Vec<Iteration>,
    }

    pub fn benchmark_run<P: Program>(args: Args<P>) -> BenchmarkResult {
        let mut result: BenchmarkResult = BenchmarkResult {
            runs: Vec::with_capacity(args.iterations),
        };
        for _ in 0..args.iterations {
            result.runs.push(run_once(&args.p));
        }
        result
    }

    fn run_once<P: Program>(program: &P) -> Iteration {
        let prepared = program.prepare();
        let wall_clock_start = SystemTime::now();
        let start = Instant::now();
        prepared.benchmark_this();
        let end = Instant::now();
        Iteration {
            started_at: wall_clock_start,
            duration: end - start,
        }
    }

}
