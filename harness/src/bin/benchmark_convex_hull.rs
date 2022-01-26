use clap::Parser;
use criterion;
use geo::{algorithm::convex_hull::ConvexHull, MultiPolygon};
use geos::{self, Geom};
use harness::{
    data::{self, MultiPolygonPack},
    notsofine::*,
};

use std::{fs::File, io::Write, time::Duration};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CLIArgs {
    #[clap(short, long)]
    geojson_file: String,
    #[clap(short, long)]
    out_file: String,
    #[clap(short, long, default_value_t = 210)]
    iterations: usize,
    #[clap(short, long)]
    headlong: bool,
}

const NUM_COMPUTATIONS: usize = 1_000;

fn geo_convex_hull(polygon: &mut MultiPolygon<f64>) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut *polygon).convex_hull());
    }
}

fn geos_convex_hull(g: &mut geos::Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut *g).convex_hull().unwrap());
    }
}

fn main() {
    let args = CLIArgs::parse();
    let MultiPolygonPack {
        geo: geo_mp,
        geos: geos_mp,
    } = data::load_multipolygon_pack(&args.geojson_file);

    println!("geo convex hull: {:#?}", geo_mp.convex_hull());
    println!(
        "geos convex hull: {:#?}",
        geos_mp.convex_hull().unwrap().to_wkt().unwrap()
    );

    let result = benchmark_run(Args {
        programs: vec![
            simple::program_for_fn_with_arg("geo", geo_convex_hull, geo_mp),
            simple::program_for_fn_with_arg("geos", geos_convex_hull, geos_mp),
        ],
        iterations: args.iterations,
        discard_leading: if args.headlong { None } else { Some(10) },
        pause: if args.headlong {
            None
        } else {
            Some(Duration::new(2, 0))
        },
    });

    File::create(args.out_file)
        .unwrap()
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
