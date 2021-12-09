use clap::Parser;
use harness::notsofine::*;
use std::{fs::File, io::Write, thread, time::Duration};

#[derive(Clone)]
pub struct Looper {
    name: String,
}

impl Program for Looper {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn prepare(&self) -> Box<dyn PreparedProgram> {
        return Box::new(self.clone());
    }
}

impl PreparedProgram for Looper {
    fn benchmark_this(&self) {
        for _ in 1..40000 {
            for _ in 1..100 {}
        }
    }
}

#[derive(Clone, Copy)]
pub struct Sleeper;

impl Program for Sleeper {
    fn name(&self) -> String {
        "Sleeper".to_owned()
    }
    fn prepare(&self) -> Box<dyn PreparedProgram> {
        return Box::new(self.clone());
    }
}

impl PreparedProgram for Sleeper {
    fn benchmark_this(&self) {
        thread::sleep(Duration::from_millis(100));
    }
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CLIArgs {
    /// Name of the person to greet
    #[clap(short, long)]
    out_file: String,

    #[clap(short, long)]
    iterations: usize,
}

fn main() {
    let args = CLIArgs::parse();
    let result = benchmark_run(Args {
        programs: vec![
            Box::new(Looper {
                name: "Looper1".to_owned(),
            }),
            Box::new(Sleeper {}),
            Box::new(Looper {
                name: "Looper2".to_owned(),
            }),
            Box::new(Looper {
                name: "Looper3".to_owned(),
            }),
        ],
        iterations: args.iterations,
    });

    let mut ofile = File::create(args.out_file).unwrap();
    ofile
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
