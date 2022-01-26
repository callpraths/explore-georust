use clap::Parser;
use criterion;
use geo::{algorithm::bounding_rect::BoundingRect, MultiPolygon};
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

const NUM_COMPUTATIONS: usize = 100_000;

fn geo_mbr(polygon: &mut MultiPolygon<f64>) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut *polygon).bounding_rect());
    }
}

fn geos_mbr(g: &mut geos::Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut *g).envelope().unwrap());
    }
}

// A sanity test that preprocessing is not the reason for fast Envelope computation.
// This should be nearly half the qps as `mbr_twice`.
fn geos_mbr_twice_cloned(g: &mut (geos::Geometry, geos::Geometry)) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut g.0).envelope().unwrap());
        criterion::black_box(criterion::black_box(&mut g.1).envelope().unwrap());
    }
}

fn geos_mbr_twice(g: &mut geos::Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut *g).envelope().unwrap());
        criterion::black_box(criterion::black_box(&mut *g).envelope().unwrap());
    }
}

fn main() {
    let args = CLIArgs::parse();
    let MultiPolygonPack {
        geo: geo_mp,
        geos: geos_mp,
    } = data::load_multipolygon_pack(&args.geojson_file);

    // Clone values **before** getting MBR because GEOS caches the value.
    let programs = vec![
        simple::program_for_fn_with_arg("geo", geo_mbr, geo_mp.clone()),
        simple::program_for_fn_with_arg("geos", geos_mbr, Geom::clone(&geos_mp)),
        simple::program_for_fn_with_arg("geos_twice", geos_mbr_twice, Geom::clone(&geos_mp)),
        simple::program_for_fn_with_arg(
            "geos_twice_cloned",
            geos_mbr_twice_cloned,
            (Geom::clone(&geos_mp), Geom::clone(&geos_mp)),
        ),
    ];
    println!("geo MBR: {:#?}", geo_mp.bounding_rect());
    println!(
        "geos MBR: {:#?}",
        geos_mp.envelope().unwrap().to_wkt().unwrap()
    );

    let result = benchmark_run(Args {
        programs,
        iterations: args.iterations,
        discard_leading: Some(20),
        pause: if args.headlong {
            None
        } else {
            Some(Duration::new(1, 0))
        },
    });

    File::create(args.out_file)
        .unwrap()
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
