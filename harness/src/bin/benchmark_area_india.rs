use clap::Parser;
use criterion;
use geo::{algorithm::area::Area, Geometry, MultiPolygon};
use geos::{self, Geom};
use geozero::{geo_types::GeoWriter, geojson::GeoJsonReader, GeozeroDatasource};
use harness::notsofine::*;
use std::{fs::File, io::Write};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct CLIArgs {
    #[clap(short, long)]
    geojson_file: String,
    #[clap(short, long)]
    out_file: String,
    #[clap(short, long)]
    iterations: usize,
}

const NUM_COMPUTATIONS: usize = 100_000;

fn geo_area(mut polygon: Geometry<f64>) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut polygon).signed_area());
    }
}

fn geos_area(mut g: geos::Geometry) {
    for _ in 0..NUM_COMPUTATIONS {
        criterion::black_box(criterion::black_box(&mut g).area().unwrap());
    }
}

fn main() {
    let args = CLIArgs::parse();
    let polygons = load(&args.geojson_file).unwrap();

    let result = benchmark_run(Args {
        programs: vec![
            simple::program_for_fn_with_arg(
                "geo",
                geo_area,
                Geometry::MultiPolygon(polygons.clone()),
            ),
            simple::program_for_fn_with_arg("geos", geos_area, polygons.try_into().unwrap()),
        ],
        iterations: args.iterations,
    });

    let mut ofile = File::create(args.out_file).unwrap();
    ofile
        .write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes())
        .unwrap();
}

fn load(path: &str) -> Result<MultiPolygon<f64>, String> {
    let mut writer = GeoWriter::new();
    let mut f = File::open(path).unwrap();
    GeoJsonReader(&mut f).process(&mut writer).unwrap();
    let geometry = writer.geometry();
    if let Geometry::MultiPolygon(p) = geometry {
        return Ok(p.clone());
    }
    Err("Loaded geometry is not a MultiPolygon".to_owned())
}
