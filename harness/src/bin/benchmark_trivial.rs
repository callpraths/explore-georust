use harness::notsofine::*;
use std::{thread, time::Duration};

#[derive(Clone, Copy)]
pub struct Looper;

impl Program for Looper {
    fn name(&self) -> String {
        "Looper".to_owned()
    }
    fn prepare(&self) -> Box<dyn PreparedProgram> {
        return Box::new(self.clone());
    }
}

impl PreparedProgram for Looper {
    fn benchmark_this(&self) {
        for _ in 1..40000 {}
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
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let result = benchmark_run(Args {
        programs: vec![Box::new(Looper {}), Box::new(Sleeper {})],
        iterations: 2,
    });
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
