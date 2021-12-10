use clap::Parser;
use geo::{algorithm::area::Area, Coordinate, Polygon};
use geos::{Geom, Geometry};
use harness::notsofine::*;
use std::{f64::consts::PI, fs::File, io::Write};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CLIArgs {
    /// Name of the person to greet
    #[clap(short, long)]
    out_file: String,

    #[clap(short, long)]
    iterations: usize,
}

const NUM_COMPUTATIONS: usize = 10_000;

fn geo_area(polygon: Polygon<f64>) {
    for _ in 0..NUM_COMPUTATIONS {
        polygon.signed_area();
    }
}

fn geos_area(g: Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        g.area().unwrap();
    }
}

fn main() {
    let args = CLIArgs::parse();

    let polygon = {
        const NUM_VERTICES: usize = 10;
        const ANGLE_INC: f64 = 2. * PI / NUM_VERTICES as f64;
        const RADIUS: f64 = 10.;

        Polygon::new(
            (0..NUM_VERTICES)
                .map(|i| {
                    let angle = i as f64 * ANGLE_INC;
                    Coordinate {
                        x: RADIUS * angle.cos(),
                        y: RADIUS * angle.sin(),
                    }
                })
                .collect::<Vec<_>>()
                .into(),
            vec![],
        )
    };

    let result = benchmark_run(Args {
        programs: vec![
            simple::program_for_fn_with_arg("geo", geo_area, polygon.clone()),
            simple::program_for_fn_with_arg("geos", geos_area, polygon.try_into().unwrap()),
        ],
        iterations: args.iterations,
    });

    let mut ofile = File::create(args.out_file).unwrap();
    ofile
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
