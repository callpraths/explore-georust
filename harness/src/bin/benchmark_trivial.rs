use harness::notsofine::*;
use std::{thread, time::Duration};

#[derive(Clone, Copy)]
pub struct Looper;

impl Program for Looper {
    type P = Looper;
    fn name() -> String {
        "Looper".to_owned()
    }
    fn prepare(&self) -> Self::P {
        return *self;
    }
}

impl PreparedProgram for Looper {
    fn benchmark_this(self) {
        for _ in 1..40000 {
        }
    }
}


#[derive(Clone, Copy)]
pub struct Sleeper;

impl Program for Sleeper {
    type P = Sleeper;
    fn name() -> String {
        "Sleeper".to_owned()
    }
    fn prepare(&self) -> Self::P {
        return *self;
    }
}

impl PreparedProgram for Sleeper {
    fn benchmark_this(self) {
        thread::sleep(Duration::from_secs(1));
    }
}


fn main() {
    println!(
        "{:#?}",
        benchmark_run(Args::<Looper> { programs: vec![Looper {}, Sleeper {}], iterations: 2 })
    );
}