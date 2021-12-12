pub mod notsofine {
    use serde::Serialize;
    use std::time::{Duration, Instant, SystemTime};

    pub trait Program {
        fn name(&self) -> String;
        fn prepare(&self) -> Box<dyn PreparedProgram>;
    }

    pub trait PreparedProgram {
        fn benchmark_this(&mut self);
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
            println!("Iteration {} of {}", i + 1, args.iterations);
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
        let mut prepared = program.prepare();

        let start = Instant::now();
        prepared.benchmark_this();
        let end = Instant::now();
        Run {
            program: program.name(),
            duration: end - start,
        }
    }

    pub mod simple {
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
    }
}

pub mod data {
    use geo::{self, Geometry, MultiPolygon};
    use geos::Geometry as GeosGeometry;
    use geozero::{geo_types::GeoWriter, geojson::GeoJsonReader, GeozeroDatasource};
    use std::fs::File;

    pub struct MultiPolygonPack<'a> {
        pub geo: Geometry<f64>,
        pub geos: GeosGeometry<'a>,
    }

    pub fn load_multipolygon_pack(path: &str) -> MultiPolygonPack<'static> {
        let p = load_multipolygon(path);
        MultiPolygonPack {
            geo: Geometry::MultiPolygon(p.clone()),
            geos: p.try_into().unwrap(),
        }
    }

    pub fn load_multipolygon(path: &str) -> MultiPolygon<f64> {
        let mut w = GeoWriter::new();
        GeoJsonReader(&mut File::open(path).unwrap())
            .process(&mut w)
            .unwrap();
        let geometry = w.geometry();
        if let Geometry::MultiPolygon(p) = geometry {
            return p.clone();
        }
        panic!("Loaded geometry from {} is not a MultiPolygon", path);
    }
}
