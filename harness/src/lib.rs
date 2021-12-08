pub mod notsofine {
    use std::time::{Duration, Instant, SystemTime};

    pub trait Program {
        type P: PreparedProgram;
        fn name() -> String;
        fn prepare(&self) -> Self::P;
    }

    pub trait PreparedProgram {
        fn benchmark_this(self);
    }

    pub struct Args<P: Program> {
        pub programs: Vec<P>,
        pub iterations: usize,
    }

    #[derive(Clone, Debug)]
    pub struct Iteration {
        pub program: String,
        pub started_at: SystemTime,
        pub duration: Duration,
    }

    #[derive(Clone, Debug)]
    pub struct BenchmarkResult {
        pub runs: Vec<Iteration>,
    }

    pub fn benchmark_run<P: Program>(args: Args<P>) -> BenchmarkResult {
        let mut result: BenchmarkResult = BenchmarkResult {
            runs: Vec::with_capacity(args.iterations * args.programs.len()),
        };

        let radix = args.programs.len();
        let mut head: usize = 0;
        for _ in 0..args.iterations {
            for i in 0..radix {
                result.runs.push(run_once(&args.programs[(head+i)%radix]));
            }
            head = (head + 1) % radix;
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
            program: P::name(),
            started_at: wall_clock_start,
            duration: end - start,
        }
    }

}
