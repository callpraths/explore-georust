use harness::notsofine::*;


#[derive(Clone, Copy)]
pub struct SingleTrickPony;

impl Program for SingleTrickPony {
    type P = SingleTrickPony;
    fn prepare(&self) -> Self::P {
        return *self;
    }
}

impl PreparedProgram for SingleTrickPony {
    fn benchmark_this(self) {
        for i in 1..40000 {
        }
    }
}

fn main() {
    let p = SingleTrickPony {};
    println!(
        "{:#?}",
        benchmark_run(Args::<SingleTrickPony> { p, iterations: 100 })
    );
}