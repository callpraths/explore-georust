use geo::{
    algorithm::area::Area, map_coords::MapCoords, CoordFloat, CoordNum, Coordinate, LineString,
    Polygon,
};
use geos::{Geom, Geometry};
use serde::Serialize;
use std::f64::consts::PI;

fn main() {
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

    let shifts = (0..64)
        .map(f64::from)
        .map(|i| 1.5 * i.exp2())
        .collect::<Vec<f64>>();
    let shift_angles = [0., PI / 8., PI / 4., PI * 3. / 8., PI / 2.];
    let series = compute_series(&polygon, &shifts, &shift_angles);
    println!("{}", serde_json::to_string_pretty(&series).unwrap());
}

#[derive(Serialize)]
struct Series {
    pub angle: f64,
    pub data: Vec<Datum>,
}

#[derive(Serialize)]
struct Datum {
    pub shift: f64,
    pub geo: ComputedArea,
    pub geos: ComputedArea,
    pub naive: ComputedArea,
    pub naive_geo_relative_error: ComputedArea,
    pub geos_geo_relative_error: ComputedArea,
    pub geo_geos_relative_error: ComputedArea,
}

#[derive(Serialize)]
struct ComputedArea {
    pub original: f64,
    pub shifted: f64,
}

fn compute_series(polygon: &Polygon<f64>, shifts: &[f64], angles: &[f64]) -> Vec<Series> {
    let mut series: Vec<Series> = Vec::with_capacity(angles.len());
    for angle in angles {
        let mut data: Vec<Datum> = Vec::with_capacity(shifts.len());

        for shift in shifts {
            data.push(compute_datum(polygon, *angle, *shift));
        }
        series.push(Series {
            angle: *angle,
            data,
        })
    }
    series
}

fn compute_datum(polygon: &Polygon<f64>, angle: f64, shift: f64) -> Datum {
    let xshift = shift * angle.cos();
    let yshift = shift * angle.sin();
    let shifted_polygon = polygon.map_coords(|&(x, y)| (x + xshift, y + yshift));

    Datum {
        shift,
        geo: ComputedArea {
            original: geo_area(polygon),
            shifted: geo_area(&shifted_polygon),
        },
        geos: ComputedArea {
            original: geos_area(polygon),
            shifted: geos_area(&shifted_polygon),
        },
        naive: ComputedArea {
            original: naive_area(polygon),
            shifted: naive_area(&shifted_polygon),
        },
        naive_geo_relative_error: ComputedArea {
            original: (geo_area(polygon) - naive_area(polygon)).abs() / geo_area(polygon),
            shifted: (geo_area(&shifted_polygon) - naive_area(&shifted_polygon)).abs()
                / geo_area(&shifted_polygon),
        },
        geos_geo_relative_error: ComputedArea {
            original: (geo_area(polygon) - geos_area(polygon)).abs() / geo_area(polygon),
            shifted: (geo_area(&shifted_polygon) - geos_area(&shifted_polygon)).abs()
                / geos_area(&shifted_polygon),
        },
        geo_geos_relative_error: ComputedArea {
            original: (geo_area(polygon) - geos_area(polygon)).abs() / geos_area(polygon),
            shifted: (geo_area(&shifted_polygon) - geos_area(&shifted_polygon)).abs()
                / geos_area(&shifted_polygon),
        },
    }
}

fn geo_area(polygon: &Polygon<f64>) -> f64 {
    polygon.signed_area()
}

fn geos_area(polygon: &Polygon<f64>) -> f64 {
    let g: Geometry = polygon.try_into().unwrap();
    g.area().unwrap()
}

fn naive_area(polygon: &Polygon<f64>) -> f64 {
    get_linestring_area_naive(polygon.exterior())
}

fn get_linestring_area_naive<T>(linestring: &LineString<T>) -> T
where
    T: CoordFloat,
{
    twice_signed_ring_area(linestring) / (T::one() + T::one())
}

fn twice_signed_ring_area<T>(linestring: &LineString<T>) -> T
where
    T: CoordNum,
{
    // LineString with less than 3 points is empty, or a
    // single point, or is not closed.
    if linestring.0.len() < 3 {
        return T::zero();
    }

    // Above test ensures the vector has at least 2 elements.
    // We check if linestring is closed, and return 0 otherwise.
    if linestring.0.first().unwrap() != linestring.0.last().unwrap() {
        return T::zero();
    }

    let mut tmp = T::zero();
    for line in linestring.lines() {
        tmp = tmp + line.determinant();
    }

    tmp
}
