use clap::Parser;
use criterion;
use geo::{algorithm::bounding_rect::BoundingRect, Geometry};
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
}

const NUM_COMPUTATIONS: usize = 100_000;

fn geo_mbr(mut polygon: Geometry<f64>) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut polygon).bounding_rect());
    }
}

fn geos_mbr(mut g: geos::Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut g).envelope().unwrap());
    }
}

fn main() {
    let args = CLIArgs::parse();
    let MultiPolygonPack {
        geo: geo_mp,
        geos: geos_mp,
    } = data::load_multipolygon_pack(&args.geojson_file);

    println!("geo MBR: {:#?}", geo_mp.bounding_rect());
    println!(
        "geos MBR: {:#?}",
        geos_mp.envelope().unwrap().to_wkt().unwrap()
    );

    let result = benchmark_run(Args {
        programs: vec![
            simple::program_for_fn_with_arg("geo", geo_mbr, geo_mp),
            simple::program_for_fn_with_arg("geos", geos_mbr, geos_mp),
        ],
        iterations: 210,
        discard_leading: Some(10),
        pause: Some(Duration::new(1, 0)),
    });

    File::create(args.out_file)
        .unwrap()
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}
