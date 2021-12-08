use harness::notsofine::*;


#[derive(Clone, Copy)]
pub struct SingleTrickPony;

impl Program for SingleTrickPony {
    type P = SingleTrickPony;
    fn name() -> String {
        "Looper".to_owned()
    }
    fn prepare(&self) -> Self::P {
        return *self;
    }
}

impl PreparedProgram for SingleTrickPony {
    fn benchmark_this(self) {
        for _ in 1..40000 {
        }
    }
}

fn main() {
    let p = SingleTrickPony {};
    println!(
        "{:#?}",
        benchmark_run(Args::<SingleTrickPony> { programs: vec![p], iterations: 2 })
    );
}