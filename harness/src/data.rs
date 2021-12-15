use geo::{self, Geometry, MultiPolygon};
use geos::Geometry as GeosGeometry;
use geozero::{geo_types::GeoWriter, geojson::GeoJsonReader, GeozeroDatasource};
use std::fs::File;

pub struct MultiPolygonPack<'a> {
    pub geo: MultiPolygon<f64>,
    pub geos: GeosGeometry<'a>,
}

pub fn load_multipolygon_pack(path: &str) -> MultiPolygonPack<'static> {
    let p = load_multipolygon(path);
    MultiPolygonPack {
        geo: p.clone(),
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
