use super::{PreparedProgram, Program};

pub fn program_for_fn(name: &str, f: fn()) -> Box<dyn Program> {
    Box::new(FnProgram {
        name: name.to_owned(),
        f,
    })
}

#[derive(Clone, Debug)]
struct FnProgram {
    name: String,
    f: fn(),
}

impl Program for FnProgram {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn prepare(&self) -> Box<dyn PreparedProgram> {
        return Box::new(self.clone());
    }
}

impl PreparedProgram for FnProgram {
    fn benchmark_this(&mut self) {
        (self.f)()
    }
}

pub fn program_for_fn_with_arg<T: Clone + 'static>(
    name: &str,
    f: fn(T),
    arg: T,
) -> Box<dyn Program> {
    Box::new(FnWithArgProgram {
        name: name.to_owned(),
        f,
        arg: Some(arg),
    })
}

#[derive(Clone, Debug)]
struct FnWithArgProgram<T: Clone> {
    name: String,
    f: fn(T),
    arg: Option<T>,
}

impl<T: Clone + 'static> Program for FnWithArgProgram<T> {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn prepare(&self) -> Box<dyn PreparedProgram> {
        return Box::new(self.clone());
    }
}

impl<T: Clone> PreparedProgram for FnWithArgProgram<T> {
    fn benchmark_this(&mut self) {
        (self.f)(self.arg.take().unwrap())
    }
}
